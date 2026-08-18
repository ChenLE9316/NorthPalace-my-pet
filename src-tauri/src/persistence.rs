use std::{
    fs,
    path::Path,
    sync::{
        mpsc::{self, RecvTimeoutError, Sender},
        Arc, RwLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    domain::{
        events::DomainEvent,
        memory::{MemoryDraft, MemoryKind, MemorySearchHit},
        pet_state::{Facing, PetStateV2},
    },
    runtime::RuntimeHandle,
    worker::WorkerSupervisor,
};

const SCHEMA_VERSION: i64 = 2;
const ACTIVITY_RETENTION_MS: i64 = 30 * 24 * 60 * 60 * 1_000;
const ACTIVITY_MAX_ROWS: i64 = 2_000;
const JOURNAL_RECV_POLL: Duration = Duration::from_millis(100);
const PERSISTENCE_RECV_POLL: Duration = Duration::from_millis(250);

#[derive(Debug, Clone, PartialEq)]
struct ActivityRecord {
    event_type: String,
    category: String,
    relationship_kind: Option<String>,
    bond_delta: f32,
    counts_as_interaction: bool,
}

impl ActivityRecord {
    fn from_domain_event(event: &DomainEvent) -> Option<Self> {
        match event {
            DomainEvent::UserReturned => Some(Self {
                event_type: "user_returned".to_owned(),
                category: "presence".to_owned(),
                relationship_kind: Some("reunion".to_owned()),
                bond_delta: 0.0,
                counts_as_interaction: true,
            }),
            DomainEvent::PetPetted => Some(Self {
                event_type: "pet_petted".to_owned(),
                category: "relationship".to_owned(),
                relationship_kind: Some("affection".to_owned()),
                bond_delta: 0.01,
                counts_as_interaction: true,
            }),
            DomainEvent::PetPlayRequested => Some(Self {
                event_type: "pet_play".to_owned(),
                category: "relationship".to_owned(),
                relationship_kind: Some("play".to_owned()),
                bond_delta: 0.005,
                counts_as_interaction: true,
            }),
            DomainEvent::FocusModeStarted => Some(Self {
                event_type: "focus_started".to_owned(),
                category: "focus".to_owned(),
                relationship_kind: None,
                bond_delta: 0.0,
                counts_as_interaction: true,
            }),
            DomainEvent::FocusModeEnded => Some(Self {
                event_type: "focus_ended".to_owned(),
                category: "focus".to_owned(),
                relationship_kind: None,
                bond_delta: 0.0,
                counts_as_interaction: true,
            }),
            _ => None,
        }
    }
}

enum PersistenceCommand {
    Save(PetStateV2),
    SaveAndFlush {
        state: PetStateV2,
        ack: mpsc::SyncSender<Result<(), String>>,
    },
    RecordActivity(ActivityRecord),
    ObserveHour {
        hour: u8,
        interaction_delta: u32,
    },
    StoreMemory(MemoryDraft),
    SearchMemories {
        query: String,
        limit: u32,
        ack: mpsc::SyncSender<Result<Vec<MemorySearchHit>, String>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct PersistentPetState {
    facing: Facing,
    energy: f32,
    curiosity: f32,
    bond: f32,
    sleep_pressure: f32,
}

impl PersistentPetState {
    fn from_runtime(state: &PetStateV2) -> Self {
        Self {
            facing: state.facing,
            energy: state.energy.clamp(0.0, 1.0),
            curiosity: state.curiosity.clamp(0.0, 1.0),
            bond: state.bond.clamp(0.0, 1.0),
            sleep_pressure: state.sleep_pressure.clamp(0.0, 1.0),
        }
    }

    fn into_runtime_state(self) -> PetStateV2 {
        let mut state = PetStateV2::default();
        state.facing = self.facing;
        state.energy = self.energy.clamp(0.0, 1.0);
        state.curiosity = self.curiosity.clamp(0.0, 1.0);
        state.bond = self.bond.clamp(0.0, 1.0);
        state.sleep_pressure = self.sleep_pressure.clamp(0.0, 1.0);
        state
    }
}

pub struct PersistenceBootstrap {
    connection: Connection,
    initial_state: PetStateV2,
    had_saved_state: bool,
}

impl PersistenceBootstrap {
    pub fn open(path: &Path) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create local data directory {}: {error}",
                    parent.display()
                )
            })?;
        }

        let connection = Connection::open(path)
            .map_err(|error| format!("failed to open SQLite database {}: {error}", path.display()))?;
        Self::from_connection(connection)
    }

    fn from_connection(connection: Connection) -> Result<Self, String> {
        configure_connection(&connection)?;
        migrate(&connection)?;
        let loaded = load_pet_state(&connection)?;
        let had_saved_state = loaded.is_some();
        let initial_state = loaded
            .map(PersistentPetState::into_runtime_state)
            .unwrap_or_default();

        Ok(Self {
            connection,
            initial_state,
            had_saved_state,
        })
    }

    pub fn initial_state(&self) -> PetStateV2 {
        self.initial_state.clone()
    }

    pub fn into_worker(self, supervisor: &WorkerSupervisor) -> Result<PersistenceHandle, String> {
        let (tx, rx) = mpsc::channel::<PersistenceCommand>();
        let connection = self.connection;
        let handle = PersistenceHandle {
            tx,
            had_saved_state: self.had_saved_state,
        };

        supervisor.spawn("persistence-db", move |token| {
            let mut connection = connection;

            loop {
                let command = match rx.recv_timeout(PERSISTENCE_RECV_POLL) {
                    Ok(command) => command,
                    Err(RecvTimeoutError::Timeout) if token.is_cancelled() => break,
                    Err(RecvTimeoutError::Timeout) => continue,
                    Err(RecvTimeoutError::Disconnected) => break,
                };

                match command {
                    PersistenceCommand::Save(state) => {
                        if let Err(error) = save_pet_state(
                            &mut connection,
                            &PersistentPetState::from_runtime(&state),
                        ) {
                            eprintln!("Lenvu persistence save failed: {error}");
                        }
                    }
                    PersistenceCommand::SaveAndFlush { state, ack } => {
                        let result = save_pet_state(
                            &mut connection,
                            &PersistentPetState::from_runtime(&state),
                        );
                        let _ = ack.send(result);
                    }
                    PersistenceCommand::RecordActivity(activity) => {
                        if let Err(error) = record_activity(&mut connection, &activity) {
                            eprintln!("Lenvu activity journal write failed: {error}");
                        }
                    }
                    PersistenceCommand::ObserveHour {
                        hour,
                        interaction_delta,
                    } => {
                        if let Err(error) = observe_hour(&mut connection, hour, interaction_delta) {
                            eprintln!("Lenvu rhythm persistence failed: {error}");
                        }
                    }
                    PersistenceCommand::StoreMemory(memory) => {
                        if let Err(error) = insert_memory(&mut connection, &memory) {
                            eprintln!("Lenvu memory write failed: {error}");
                        }
                    }
                    PersistenceCommand::SearchMemories { query, limit, ack } => {
                        let result = search_memories(&connection, &query, limit);
                        let _ = ack.send(result);
                    }
                }
            }

            Ok(())
        })?;

        Ok(handle)
    }
}

#[derive(Clone)]
pub struct PersistenceHandle {
    tx: Sender<PersistenceCommand>,
    had_saved_state: bool,
}

impl PersistenceHandle {
    pub fn queue_save(&self, state: PetStateV2) -> Result<(), String> {
        self.tx
            .send(PersistenceCommand::Save(state))
            .map_err(|_| "persistence worker channel is unavailable".to_owned())
    }

    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {
        let (ack_tx, ack_rx) = mpsc::sync_channel(1);
        self.tx
            .send(PersistenceCommand::SaveAndFlush { state, ack: ack_tx })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;

        ack_rx
            .recv_timeout(timeout)
            .map_err(|error| format!("persistence final-save acknowledgement failed: {error}"))??;
        Ok(())
    }

    fn queue_activity(&self, activity: ActivityRecord) -> Result<(), String> {
        self.tx
            .send(PersistenceCommand::RecordActivity(activity))
            .map_err(|_| "persistence worker channel is unavailable".to_owned())
    }

    fn observe_hour(&self, hour: u8, interaction_delta: u32) -> Result<(), String> {
        self.tx
            .send(PersistenceCommand::ObserveHour {
                hour: hour.min(23),
                interaction_delta,
            })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())
    }

    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
        self.tx
            .send(PersistenceCommand::StoreMemory(memory))
            .map_err(|_| "persistence worker channel is unavailable".to_owned())
    }

    pub fn search_memories(
        &self,
        query: impl Into<String>,
        limit: u32,
        timeout: Duration,
    ) -> Result<Vec<MemorySearchHit>, String> {
        let (ack_tx, ack_rx) = mpsc::sync_channel(1);
        self.tx
            .send(PersistenceCommand::SearchMemories {
                query: query.into(),
                limit: limit.clamp(1, 100),
                ack: ack_tx,
            })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;

        ack_rx
            .recv_timeout(timeout)
            .map_err(|error| format!("memory search acknowledgement failed: {error}"))?
    }

    pub fn had_saved_state(&self) -> bool {
        self.had_saved_state
    }
}

#[derive(Clone, Default)]
pub struct PersistenceService {
    inner: Arc<RwLock<Option<PersistenceHandle>>>,
}

impl PersistenceService {
    pub fn install(&self, handle: PersistenceHandle) -> Result<(), String> {
        self.inner
            .write()
            .map(|mut slot| *slot = Some(handle))
            .map_err(|_| "persistence service lock is poisoned".to_owned())
    }

    fn handle(&self) -> Option<PersistenceHandle> {
        self.inner.read().ok().and_then(|slot| slot.clone())
    }

    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.save_and_flush(state, timeout)
    }

    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.queue_memory(memory)
    }

    pub fn search_memories(
        &self,
        query: impl Into<String>,
        limit: u32,
        timeout: Duration,
    ) -> Result<Vec<MemorySearchHit>, String> {
        let Some(handle) = self.handle() else {
            return Ok(Vec::new());
        };
        handle.search_memories(query, limit, timeout)
    }

    fn queue_activity(&self, activity: ActivityRecord) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.queue_activity(activity)
    }

    fn observe_hour(&self, hour: u8, interaction_delta: u32) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.observe_hour(hour, interaction_delta)
    }
}

pub fn spawn_autosave(
    runtime: RuntimeHandle,
    persistence: PersistenceHandle,
    interval: Duration,
    supervisor: &WorkerSupervisor,
) -> Result<(), String> {
    supervisor.spawn("persistence-autosave", move |token| {
        let mut last_queued: Option<PersistentPetState> = None;

        while !token.wait_timeout(interval) {
            let Ok(snapshot) = runtime.snapshot() else {
                continue;
            };
            let persistent = PersistentPetState::from_runtime(&snapshot.state);
            if last_queued.as_ref() == Some(&persistent) {
                continue;
            }

            if let Err(error) = persistence.queue_save(snapshot.state) {
                if token.is_cancelled() {
                    break;
                }
                return Err(error);
            }
            last_queued = Some(persistent);
        }

        Ok(())
    })
}

pub fn spawn_event_journal(
    runtime: RuntimeHandle,
    persistence: PersistenceService,
    supervisor: &WorkerSupervisor,
) -> Result<(), String> {
    let events = runtime.subscribe_events();

    supervisor.spawn("persistence-event-journal", move |token| {
        let mut current_hour: Option<u8> = None;

        loop {
            let event = match events.recv_timeout(JOURNAL_RECV_POLL) {
                Ok(event) => event,
                Err(RecvTimeoutError::Timeout) if token.is_cancelled() => break,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => {
                    if token.is_cancelled() {
                        break;
                    }
                    return Err("pet runtime event subscription disconnected".to_owned());
                }
            };

            if let DomainEvent::TimeOfDayChanged { hour } = &event {
                let hour = (*hour).min(23);
                current_hour = Some(hour);
                persistence.observe_hour(hour, 0)?;
                continue;
            }

            let Some(activity) = ActivityRecord::from_domain_event(&event) else {
                continue;
            };
            let counts_as_interaction = activity.counts_as_interaction;
            persistence.queue_activity(activity)?;

            if counts_as_interaction {
                if let Some(hour) = current_hour {
                    persistence.observe_hour(hour, 1)?;
                }
            }
        }

        Ok(())
    })
}

fn configure_connection(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "PRAGMA foreign_keys = ON;\n\
             PRAGMA journal_mode = WAL;\n\
             PRAGMA synchronous = NORMAL;\n\
             PRAGMA busy_timeout = 2500;",
        )
        .map_err(|error| format!("failed to configure SQLite: {error}"))
}

fn migrate(connection: &Connection) -> Result<(), String> {
    let version: i64 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| format!("failed to read SQLite schema version: {error}"))?;

    match version {
        0 => connection
            .execute_batch(
                "BEGIN IMMEDIATE;\n\
                 CREATE TABLE IF NOT EXISTS pet_state (\n\
                   singleton INTEGER PRIMARY KEY CHECK (singleton = 1),\n\
                   facing TEXT NOT NULL CHECK (facing IN ('left', 'right')),\n\
                   energy REAL NOT NULL CHECK (energy >= 0.0 AND energy <= 1.0),\n\
                   curiosity REAL NOT NULL CHECK (curiosity >= 0.0 AND curiosity <= 1.0),\n\
                   bond REAL NOT NULL CHECK (bond >= 0.0 AND bond <= 1.0),\n\
                   sleep_pressure REAL NOT NULL CHECK (sleep_pressure >= 0.0 AND sleep_pressure <= 1.0),\n\
                   updated_at_ms INTEGER NOT NULL\n\
                 );\n\
                 CREATE TABLE IF NOT EXISTS activity_journal (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   event_type TEXT NOT NULL,\n\
                   category TEXT NOT NULL,\n\
                   created_at_ms INTEGER NOT NULL\n\
                 );\n\
                 CREATE INDEX IF NOT EXISTS idx_activity_journal_created_at\n\
                   ON activity_journal(created_at_ms);\n\
                 CREATE INDEX IF NOT EXISTS idx_activity_journal_category_created_at\n\
                   ON activity_journal(category, created_at_ms);\n\
                 CREATE TABLE IF NOT EXISTS relationship_events (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   journal_id INTEGER,\n\
                   kind TEXT NOT NULL,\n\
                   bond_delta REAL NOT NULL DEFAULT 0.0,\n\
                   created_at_ms INTEGER NOT NULL,\n\
                   FOREIGN KEY(journal_id) REFERENCES activity_journal(id) ON DELETE SET NULL\n\
                 );\n\
                 CREATE INDEX IF NOT EXISTS idx_relationship_events_created_at\n\
                   ON relationship_events(created_at_ms);\n\
                 CREATE TABLE IF NOT EXISTS memories (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   kind TEXT NOT NULL CHECK (kind IN ('episodic', 'semantic', 'preference', 'relationship')),\n\
                   content TEXT NOT NULL,\n\
                   importance REAL NOT NULL CHECK (importance >= 0.0 AND importance <= 1.0),\n\
                   source_event_id INTEGER,\n\
                   created_at_ms INTEGER NOT NULL,\n\
                   updated_at_ms INTEGER NOT NULL,\n\
                   last_accessed_at_ms INTEGER,\n\
                   FOREIGN KEY(source_event_id) REFERENCES activity_journal(id) ON DELETE SET NULL\n\
                 );\n\
                 CREATE INDEX IF NOT EXISTS idx_memories_kind_updated_at\n\
                   ON memories(kind, updated_at_ms);\n\
                 CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts USING fts5(\n\
                   content,\n\
                   content='memories',\n\
                   content_rowid='id',\n\
                   tokenize='unicode61'\n\
                 );\n\
                 CREATE TRIGGER IF NOT EXISTS memories_ai AFTER INSERT ON memories BEGIN\n\
                   INSERT INTO memory_fts(rowid, content) VALUES (new.id, new.content);\n\
                 END;\n\
                 CREATE TRIGGER IF NOT EXISTS memories_ad AFTER DELETE ON memories BEGIN\n\
                   INSERT INTO memory_fts(memory_fts, rowid, content) VALUES ('delete', old.id, old.content);\n\
                 END;\n\
                 CREATE TRIGGER IF NOT EXISTS memories_au AFTER UPDATE ON memories BEGIN\n\
                   INSERT INTO memory_fts(memory_fts, rowid, content) VALUES ('delete', old.id, old.content);\n\
                   INSERT INTO memory_fts(rowid, content) VALUES (new.id, new.content);\n\
                 END;\n\
                 CREATE TABLE IF NOT EXISTS rhythm_hourly (\n\
                   hour INTEGER PRIMARY KEY CHECK (hour >= 0 AND hour <= 23),\n\
                   interaction_count INTEGER NOT NULL DEFAULT 0,\n\
                   last_seen_at_ms INTEGER NOT NULL\n\
                 );\n\
                 PRAGMA user_version = 2;\n\
                 COMMIT;",
            )
            .map_err(|error| format!("failed to create SQLite schema: {error}")),
        1 => connection
            .execute_batch(
                "BEGIN IMMEDIATE;\n\
                 CREATE TABLE IF NOT EXISTS activity_journal (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   event_type TEXT NOT NULL,\n\
                   category TEXT NOT NULL,\n\
                   created_at_ms INTEGER NOT NULL\n\
                 );\n\
                 CREATE INDEX IF NOT EXISTS idx_activity_journal_created_at\n\
                   ON activity_journal(created_at_ms);\n\
                 CREATE INDEX IF NOT EXISTS idx_activity_journal_category_created_at\n\
                   ON activity_journal(category, created_at_ms);\n\
                 CREATE TABLE IF NOT EXISTS relationship_events (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   journal_id INTEGER,\n\
                   kind TEXT NOT NULL,\n\
                   bond_delta REAL NOT NULL DEFAULT 0.0,\n\
                   created_at_ms INTEGER NOT NULL,\n\
                   FOREIGN KEY(journal_id) REFERENCES activity_journal(id) ON DELETE SET NULL\n\
                 );\n\
                 CREATE INDEX IF NOT EXISTS idx_relationship_events_created_at\n\
                   ON relationship_events(created_at_ms);\n\
                 CREATE TABLE IF NOT EXISTS memories (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   kind TEXT NOT NULL CHECK (kind IN ('episodic', 'semantic', 'preference', 'relationship')),\n\
                   content TEXT NOT NULL,\n\
                   importance REAL NOT NULL CHECK (importance >= 0.0 AND importance <= 1.0),\n\
                   source_event_id INTEGER,\n\
                   created_at_ms INTEGER NOT NULL,\n\
                   updated_at_ms INTEGER NOT NULL,\n\
                   last_accessed_at_ms INTEGER,\n\
                   FOREIGN KEY(source_event_id) REFERENCES activity_journal(id) ON DELETE SET NULL\n\
                 );\n\
                 CREATE INDEX IF NOT EXISTS idx_memories_kind_updated_at\n\
                   ON memories(kind, updated_at_ms);\n\
                 CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts USING fts5(\n\
                   content,\n\
                   content='memories',\n\
                   content_rowid='id',\n\
                   tokenize='unicode61'\n\
                 );\n\
                 CREATE TRIGGER IF NOT EXISTS memories_ai AFTER INSERT ON memories BEGIN\n\
                   INSERT INTO memory_fts(rowid, content) VALUES (new.id, new.content);\n\
                 END;\n\
                 CREATE TRIGGER IF NOT EXISTS memories_ad AFTER DELETE ON memories BEGIN\n\
                   INSERT INTO memory_fts(memory_fts, rowid, content) VALUES ('delete', old.id, old.content);\n\
                 END;\n\
                 CREATE TRIGGER IF NOT EXISTS memories_au AFTER UPDATE ON memories BEGIN\n\
                   INSERT INTO memory_fts(memory_fts, rowid, content) VALUES ('delete', old.id, old.content);\n\
                   INSERT INTO memory_fts(rowid, content) VALUES (new.id, new.content);\n\
                 END;\n\
                 CREATE TABLE IF NOT EXISTS rhythm_hourly (\n\
                   hour INTEGER PRIMARY KEY CHECK (hour >= 0 AND hour <= 23),\n\
                   interaction_count INTEGER NOT NULL DEFAULT 0,\n\
                   last_seen_at_ms INTEGER NOT NULL\n\
                 );\n\
                 PRAGMA user_version = 2;\n\
                 COMMIT;",
            )
            .map_err(|error| format!("failed to migrate SQLite schema v1 -> v2: {error}")),
        SCHEMA_VERSION => Ok(()),
        other => Err(format!(
            "unsupported SQLite schema version {other}; expected {SCHEMA_VERSION}"
        )),
    }
}

fn load_pet_state(connection: &Connection) -> Result<Option<PersistentPetState>, String> {
    connection
        .query_row(
            "SELECT facing, energy, curiosity, bond, sleep_pressure\n\
             FROM pet_state WHERE singleton = 1",
            [],
            |row| {
                let facing: String = row.get(0)?;
                Ok(PersistentPetState {
                    facing: if facing == "left" {
                        Facing::Left
                    } else {
                        Facing::Right
                    },
                    energy: row.get(1)?,
                    curiosity: row.get(2)?,
                    bond: row.get(3)?,
                    sleep_pressure: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|error| format!("failed to load pet state: {error}"))
}

fn save_pet_state(
    connection: &mut Connection,
    state: &PersistentPetState,
) -> Result<(), String> {
    let facing = match state.facing {
        Facing::Left => "left",
        Facing::Right => "right",
    };
    let updated_at_ms = now_ms();

    connection
        .execute(
            "INSERT INTO pet_state (\n\
               singleton, facing, energy, curiosity, bond, sleep_pressure, updated_at_ms\n\
             ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)\n\
             ON CONFLICT(singleton) DO UPDATE SET\n\
               facing = excluded.facing,\n\
               energy = excluded.energy,\n\
               curiosity = excluded.curiosity,\n\
               bond = excluded.bond,\n\
               sleep_pressure = excluded.sleep_pressure,\n\
               updated_at_ms = excluded.updated_at_ms",
            params![
                facing,
                state.energy,
                state.curiosity,
                state.bond,
                state.sleep_pressure,
                updated_at_ms
            ],
        )
        .map(|_| ())
        .map_err(|error| format!("failed to save pet state: {error}"))
}

fn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<(), String> {
    let created_at_ms = now_ms();
    let transaction = connection
        .transaction()
        .map_err(|error| format!("failed to begin activity transaction: {error}"))?;

    transaction
        .execute(
            "INSERT INTO activity_journal (event_type, category, created_at_ms)\n\
             VALUES (?1, ?2, ?3)",
            params![activity.event_type, activity.category, created_at_ms],
        )
        .map_err(|error| format!("failed to insert activity journal row: {error}"))?;

    let journal_id = transaction.last_insert_rowid();
    if let Some(kind) = &activity.relationship_kind {
        transaction
            .execute(
                "INSERT INTO relationship_events (journal_id, kind, bond_delta, created_at_ms)\n\
                 VALUES (?1, ?2, ?3, ?4)",
                params![journal_id, kind, activity.bond_delta, created_at_ms],
            )
            .map_err(|error| format!("failed to insert relationship event: {error}"))?;
    }

    let cutoff_ms = created_at_ms.saturating_sub(ACTIVITY_RETENTION_MS);
    transaction
        .execute(
            "DELETE FROM activity_journal WHERE created_at_ms < ?1",
            params![cutoff_ms],
        )
        .map_err(|error| format!("failed to prune expired activity rows: {error}"))?;
    transaction
        .execute(
            "DELETE FROM activity_journal\n\
             WHERE id IN (\n\
               SELECT id FROM activity_journal\n\
               ORDER BY id DESC\n\
               LIMIT -1 OFFSET ?1\n\
             )",
            params![ACTIVITY_MAX_ROWS],
        )
        .map_err(|error| format!("failed to enforce activity row cap: {error}"))?;

    transaction
        .commit()
        .map_err(|error| format!("failed to commit activity transaction: {error}"))
}

fn observe_hour(
    connection: &mut Connection,
    hour: u8,
    interaction_delta: u32,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO rhythm_hourly (hour, interaction_count, last_seen_at_ms)\n\
             VALUES (?1, ?2, ?3)\n\
             ON CONFLICT(hour) DO UPDATE SET\n\
               interaction_count = rhythm_hourly.interaction_count + excluded.interaction_count,\n\
               last_seen_at_ms = excluded.last_seen_at_ms",
            params![hour.min(23), interaction_delta, now_ms()],
        )
        .map(|_| ())
        .map_err(|error| format!("failed to update hourly rhythm: {error}"))
}

fn insert_memory(connection: &mut Connection, memory: &MemoryDraft) -> Result<i64, String> {
    let now = now_ms();
    connection
        .execute(
            "INSERT INTO memories (\n\
               kind, content, importance, source_event_id, created_at_ms, updated_at_ms\n\
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![
                memory.kind.as_str(),
                memory.content,
                memory.importance.clamp(0.0, 1.0),
                memory.source_event_id,
                now
            ],
        )
        .map_err(|error| format!("failed to insert memory: {error}"))?;
    Ok(connection.last_insert_rowid())
}

fn search_memories(
    connection: &Connection,
    query: &str,
    limit: u32,
) -> Result<Vec<MemorySearchHit>, String> {
    let query = query.trim();
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let mut statement = connection
        .prepare(
            "SELECT m.id, m.kind, m.content, m.importance, m.created_at_ms\n\
             FROM memory_fts\n\
             JOIN memories m ON m.id = memory_fts.rowid\n\
             WHERE memory_fts MATCH ?1\n\
             ORDER BY bm25(memory_fts), m.importance DESC, m.updated_at_ms DESC\n\
             LIMIT ?2",
        )
        .map_err(|error| format!("failed to prepare memory search: {error}"))?;

    let rows = statement
        .query_map(params![query, limit.clamp(1, 100)], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, f32>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|error| format!("failed to execute memory search: {error}"))?;

    let mut hits = Vec::new();
    for row in rows {
        let (id, kind, content, importance, created_at_ms) =
            row.map_err(|error| format!("failed to read memory search row: {error}"))?;
        let Some(kind) = MemoryKind::from_str(&kind) else {
            continue;
        };
        hits.push(MemorySearchHit {
            id,
            kind,
            content,
            importance,
            created_at_ms,
        });
    }
    Ok(hits)
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_and_pet_state_round_trip() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("bootstrap");
        assert!(!bootstrap.had_saved_state);

        let mut connection = bootstrap.connection;
        let expected = PersistentPetState {
            facing: Facing::Left,
            energy: 0.55,
            curiosity: 0.73,
            bond: 0.41,
            sleep_pressure: 0.22,
        };
        save_pet_state(&mut connection, &expected).expect("save");
        let loaded = load_pet_state(&connection).expect("load").expect("saved row");
        assert_eq!(loaded, expected);

        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("schema version");
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn loaded_state_resets_transient_runtime_fields() {
        let persistent = PersistentPetState {
            facing: Facing::Left,
            energy: 0.5,
            curiosity: 0.6,
            bond: 0.7,
            sleep_pressure: 0.2,
        };
        let runtime = persistent.into_runtime_state();
        let defaults = PetStateV2::default();

        assert_eq!(runtime.facing, Facing::Left);
        assert_eq!(runtime.energy, 0.5);
        assert_eq!(runtime.locomotion, defaults.locomotion);
        assert_eq!(runtime.posture, defaults.posture);
        assert_eq!(runtime.mode, defaults.mode);
        assert_eq!(runtime.user_idle_ms, 0);
        assert!(!runtime.ai_available);
    }

    #[test]
    fn final_save_waits_for_worker_acknowledgement() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("bootstrap");
        let supervisor = WorkerSupervisor::default();
        let persistence = bootstrap
            .into_worker(&supervisor)
            .expect("persistence worker");

        let mut state = PetStateV2::default();
        state.bond = 0.91;
        persistence
            .save_and_flush(state, Duration::from_secs(1))
            .expect("final save acknowledgement");

        let report = supervisor.shutdown_and_join(Duration::from_secs(1));
        assert_eq!(report.joined, vec!["persistence-db".to_owned()]);
        assert!(report.detached.is_empty());
    }

    #[test]
    fn journal_ignores_high_frequency_events() {
        assert!(ActivityRecord::from_domain_event(&DomainEvent::CursorEnteredPet).is_none());
        assert!(ActivityRecord::from_domain_event(&DomainEvent::PetFacingChanged {
            facing: Facing::Left,
        })
        .is_none());
        assert!(ActivityRecord::from_domain_event(&DomainEvent::PetPetted).is_some());
    }

    #[test]
    fn v2_schema_supports_relationship_memory_fts_and_rhythm() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("bootstrap");
        let mut connection = bootstrap.connection;

        let activity = ActivityRecord::from_domain_event(&DomainEvent::PetPetted)
            .expect("relationship activity");
        record_activity(&mut connection, &activity).expect("record activity");

        let relationship_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM relationship_events", [], |row| row.get(0))
            .expect("relationship count");
        assert_eq!(relationship_count, 1);

        let memory = MemoryDraft {
            kind: MemoryKind::Episodic,
            content: "The user enjoyed petting Lenvu during a quiet break".to_owned(),
            importance: 0.8,
            source_event_id: None,
        };
        insert_memory(&mut connection, &memory).expect("insert memory");
        let hits = search_memories(&connection, "petting", 10).expect("search memory");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].kind, MemoryKind::Episodic);

        observe_hour(&mut connection, 10, 1).expect("observe hour");
        observe_hour(&mut connection, 10, 2).expect("observe hour again");
        let interactions: i64 = connection
            .query_row(
                "SELECT interaction_count FROM rhythm_hourly WHERE hour = 10",
                [],
                |row| row.get(0),
            )
            .expect("rhythm row");
        assert_eq!(interactions, 3);
    }

    #[test]
    fn migration_from_v1_keeps_pet_state_and_adds_v2_tables() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        connection
            .execute_batch(
                "CREATE TABLE pet_state (\n\
                   singleton INTEGER PRIMARY KEY CHECK (singleton = 1),\n\
                   facing TEXT NOT NULL,\n\
                   energy REAL NOT NULL,\n\
                   curiosity REAL NOT NULL,\n\
                   bond REAL NOT NULL,\n\
                   sleep_pressure REAL NOT NULL,\n\
                   updated_at_ms INTEGER NOT NULL\n\
                 );\n\
                 INSERT INTO pet_state VALUES (1, 'left', 0.7, 0.6, 0.5, 0.4, 1);\n\
                 PRAGMA user_version = 1;",
            )
            .expect("seed v1 schema");

        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("migrate v1");
        assert_eq!(bootstrap.initial_state().facing, Facing::Left);
        assert_eq!(bootstrap.initial_state().bond, 0.5);

        let version: i64 = bootstrap
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("schema version");
        assert_eq!(version, SCHEMA_VERSION);
    }
}
