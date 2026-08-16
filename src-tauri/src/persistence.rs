use std::{
    fs,
    path::Path,
    sync::mpsc::{self, Sender},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    domain::pet_state::{Facing, PetStateV2},
    runtime::RuntimeHandle,
};

const SCHEMA_VERSION: i64 = 1;

enum PersistenceCommand {
    Save(PetStateV2),
    SaveAndFlush {
        state: PetStateV2,
        ack: mpsc::SyncSender<Result<(), String>>,
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

    pub fn into_worker(self) -> PersistenceHandle {
        let (tx, rx) = mpsc::channel::<PersistenceCommand>();
        let connection = self.connection;

        thread::spawn(move || {
            let mut connection = connection;
            while let Ok(command) = rx.recv() {
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
                }
            }
        });

        PersistenceHandle {
            tx,
            had_saved_state: self.had_saved_state,
        }
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

    pub fn had_saved_state(&self) -> bool {
        self.had_saved_state
    }
}

pub fn spawn_autosave(
    runtime: RuntimeHandle,
    persistence: PersistenceHandle,
    interval: Duration,
) {
    thread::spawn(move || {
        let mut last_queued: Option<PersistentPetState> = None;

        loop {
            thread::sleep(interval);
            let Ok(snapshot) = runtime.snapshot() else {
                continue;
            };
            let persistent = PersistentPetState::from_runtime(&snapshot.state);
            if last_queued.as_ref() == Some(&persistent) {
                continue;
            }

            if persistence.queue_save(snapshot.state).is_err() {
                break;
            }
            last_queued = Some(persistent);
        }
    });
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
                 PRAGMA user_version = 1;\n\
                 COMMIT;",
            )
            .map_err(|error| format!("failed to create SQLite schema: {error}")),
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
    let updated_at_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64;

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
        let persistence = bootstrap.into_worker();

        let mut state = PetStateV2::default();
        state.bond = 0.91;
        persistence
            .save_and_flush(state, Duration::from_secs(1))
            .expect("final save acknowledgement");
    }
}
