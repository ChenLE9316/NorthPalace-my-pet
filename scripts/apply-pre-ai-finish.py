from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def write(path: str, content: str) -> None:
    target = ROOT / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content, encoding="utf-8", newline="\n")


def replace(path: str, old: str, new: str, expected: int = 1) -> None:
    content = read(path)
    count = content.count(old)
    if count != expected:
        raise RuntimeError(f"{path}: expected {expected} match(es), found {count}: {old[:120]!r}")
    write(path, content.replace(old, new))


def insert_before(path: str, marker: str, addition: str) -> None:
    content = read(path)
    count = content.count(marker)
    if count != 1:
        raise RuntimeError(f"{path}: expected one marker, found {count}: {marker[:120]!r}")
    write(path, content.replace(marker, addition + marker, 1))


write(
    "src-tauri/src/domain/memory_evaluator.rs",
    '''use crate::domain::{
    events::DomainEvent,
    memory::{MemoryDraft, MemoryKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryDecision {
    StoreOrMerge(MemoryDraft),
    Discard,
}

pub fn evaluate(event: &DomainEvent) -> MemoryDecision {
    let draft = match event {
        DomainEvent::UserReturned => MemoryDraft {
            kind: MemoryKind::Relationship,
            content: "The user returns to Lenvu after being away.".to_owned(),
            importance: 0.45,
            source_event_id: None,
        },
        DomainEvent::PetPetted => MemoryDraft {
            kind: MemoryKind::Relationship,
            content: "The user shows affection by petting Lenvu.".to_owned(),
            importance: 0.70,
            source_event_id: None,
        },
        DomainEvent::PetPlayRequested => MemoryDraft {
            kind: MemoryKind::Relationship,
            content: "The user enjoys playing with Lenvu.".to_owned(),
            importance: 0.65,
            source_event_id: None,
        },
        DomainEvent::FocusModeStarted => MemoryDraft {
            kind: MemoryKind::Preference,
            content: "The user uses Focus Guard for quiet focus sessions.".to_owned(),
            importance: 0.75,
            source_event_id: None,
        },
        _ => return MemoryDecision::Discard,
    };

    MemoryDecision::StoreOrMerge(draft)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_signal_events_become_candidates() {
        for event in [
            DomainEvent::UserReturned,
            DomainEvent::PetPetted,
            DomainEvent::PetPlayRequested,
            DomainEvent::FocusModeStarted,
        ] {
            assert!(matches!(evaluate(&event), MemoryDecision::StoreOrMerge(_)));
        }
    }

    #[test]
    fn noisy_or_terminal_events_are_discarded() {
        for event in [
            DomainEvent::CursorEnteredPet,
            DomainEvent::CursorLeftPet,
            DomainEvent::PetTouched,
            DomainEvent::FocusModeEnded,
            DomainEvent::NotificationReceived,
        ] {
            assert_eq!(evaluate(&event), MemoryDecision::Discard);
        }
    }
}
''',
)

replace(
    "src-tauri/src/domain/mod.rs",
    "pub mod memory;\n",
    "pub mod memory;\npub mod memory_evaluator;\n",
)

replace("src-tauri/src/persistence.rs", "    path::Path,\n", "    path::{Path, PathBuf},\n")
replace(
    "src-tauri/src/persistence.rs",
    "        memory::{MemoryDraft, MemoryKind, MemorySearchHit},\n",
    "        memory::{MemoryDraft, MemoryKind, MemorySearchHit},\n        memory_evaluator::{self, MemoryDecision},\n",
)
replace(
    "src-tauri/src/persistence.rs",
    "    RecordActivity(ActivityRecord),\n",
    "    RecordActivity {\n        activity: ActivityRecord,\n        memory: MemoryDecision,\n    },\n",
)
replace(
    "src-tauri/src/persistence.rs",
    "    ListMemoryRecords {\n",
    "    Backup {\n        path: PathBuf,\n        ack: mpsc::SyncSender<Result<(), String>>,\n    },\n    ListMemoryRecords {\n",
)
replace(
    "src-tauri/src/persistence.rs",
    '''                    PersistenceCommand::RecordActivity(activity) => {
                        if let Err(error) = record_activity(&mut connection, &activity) {
                            eprintln!("Lenvu activity journal write failed: {error}");
                        }
                    }
''',
    '''                    PersistenceCommand::RecordActivity { activity, memory } => {
                        match record_activity(&mut connection, &activity) {
                            Ok(journal_id) => {
                                if let Err(error) =
                                    apply_memory_decision(&mut connection, journal_id, memory)
                                {
                                    eprintln!("Lenvu memory evaluator write failed: {error}");
                                }
                            }
                            Err(error) => {
                                eprintln!("Lenvu activity journal write failed: {error}");
                            }
                        }
                    }
''',
)
replace(
    "src-tauri/src/persistence.rs",
    '''                    PersistenceCommand::SearchMemories { query, limit, ack } => {
                        let result = search_memories(&connection, &query, limit);
                        let _ = ack.send(result);
                    }
                    PersistenceCommand::ListMemoryRecords { kind, limit, ack } => {
''',
    '''                    PersistenceCommand::SearchMemories { query, limit, ack } => {
                        let result = search_memories(&connection, &query, limit);
                        let _ = ack.send(result);
                    }
                    PersistenceCommand::Backup { path, ack } => {
                        let _ = ack.send(backup_database(&connection, &path));
                    }
                    PersistenceCommand::ListMemoryRecords { kind, limit, ack } => {
''',
)
replace(
    "src-tauri/src/persistence.rs",
    '''    fn queue_activity(&self, activity: ActivityRecord) -> Result<(), String> {
        self.tx
            .send(PersistenceCommand::RecordActivity(activity))
            .map_err(|_| "persistence worker channel is unavailable".to_owned())
    }
''',
    '''    fn queue_activity(
        &self,
        activity: ActivityRecord,
        memory: MemoryDecision,
    ) -> Result<(), String> {
        self.tx
            .send(PersistenceCommand::RecordActivity { activity, memory })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())
    }
''',
)
replace(
    "src-tauri/src/persistence.rs",
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {
        let (ack_tx, ack_rx) = mpsc::sync_channel(1);
        self.tx
            .send(PersistenceCommand::SaveAndFlush { state, ack: ack_tx })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;
        wait_for_ack(ack_rx, timeout, "persistence final-save")
    }

    fn queue_activity(
''',
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {
        let (ack_tx, ack_rx) = mpsc::sync_channel(1);
        self.tx
            .send(PersistenceCommand::SaveAndFlush { state, ack: ack_tx })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;
        wait_for_ack(ack_rx, timeout, "persistence final-save")
    }

    pub fn backup_to(&self, path: PathBuf, timeout: Duration) -> Result<(), String> {
        let (ack_tx, ack_rx) = mpsc::sync_channel(1);
        self.tx
            .send(PersistenceCommand::Backup { path, ack: ack_tx })
            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;
        wait_for_ack(ack_rx, timeout, "persistence backup")
    }

    fn queue_activity(
''',
)
replace(
    "src-tauri/src/persistence.rs",
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.save_and_flush(state, timeout)
    }

    #[allow(dead_code)]
''',
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.save_and_flush(state, timeout)
    }

    pub fn backup_to(&self, path: PathBuf, timeout: Duration) -> Result<(), String> {
        self.required_handle("persistent memory is unavailable for this session")?
            .backup_to(path, timeout)
    }

    #[allow(dead_code)]
''',
)
replace(
    "src-tauri/src/persistence.rs",
    '''            let counts_as_interaction = activity.counts_as_interaction;
            persistence.queue_activity(activity)?;

            if counts_as_interaction && let Some(hour) = current_hour {
''',
    '''            let counts_as_interaction = activity.counts_as_interaction;
            let memory_decision = memory_evaluator::evaluate(&event);
            persistence.queue_activity(activity, memory_decision)?;

            if counts_as_interaction && let Some(hour) = current_hour {
''',
)
replace(
    "src-tauri/src/persistence.rs",
    "fn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<(), String> {\n",
    "fn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<i64, String> {\n",
)
replace(
    "src-tauri/src/persistence.rs",
    '''    transaction
        .commit()
        .map_err(|error| format!("failed to commit activity transaction: {error}"))
}

fn observe_hour(
''',
    '''    transaction
        .commit()
        .map_err(|error| format!("failed to commit activity transaction: {error}"))?;
    Ok(journal_id)
}

fn apply_memory_decision(
    connection: &mut Connection,
    journal_id: i64,
    decision: MemoryDecision,
) -> Result<(), String> {
    let MemoryDecision::StoreOrMerge(mut memory) = decision else {
        return Ok(());
    };

    memory.source_event_id = Some(journal_id);
    let existing = connection
        .query_row(
            "SELECT id, importance FROM memories WHERE kind = ?1 AND content = ?2 ORDER BY updated_at_ms DESC, id DESC LIMIT 1",
            params![memory.kind.as_str(), memory.content],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, f32>(1)?)),
        )
        .optional()
        .map_err(|error| format!("failed to inspect memory merge candidate: {error}"))?;

    if let Some((id, existing_importance)) = existing {
        connection
            .execute(
                "UPDATE memories SET importance = ?1, source_event_id = ?2, updated_at_ms = ?3 WHERE id = ?4",
                params![existing_importance.max(memory.importance), journal_id, now_ms(), id],
            )
            .map(|_| ())
            .map_err(|error| format!("failed to merge automatic memory: {error}"))
    } else {
        insert_memory(connection, &memory).map(|_| ())
    }
}

fn backup_database(connection: &Connection, path: &Path) -> Result<(), String> {
    if path.exists() {
        return Err(format!("backup destination already exists: {}", path.display()));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create backup directory {}: {error}", parent.display())
        })?;
    }

    let destination = path.to_string_lossy().into_owned();
    connection
        .execute("VACUUM main INTO ?1", params![destination])
        .map(|_| ())
        .map_err(|error| format!("failed to create SQLite backup {}: {error}", path.display()))
}

fn observe_hour(
''',
)
replace(
    "src-tauri/src/persistence.rs",
    '''        handle
            .queue_activity(ActivityRecord {
                event_type: "pet_petted".to_owned(),
                category: "relationship".to_owned(),
                relationship_kind: Some("affection".to_owned()),
                bond_delta: 0.01,
                counts_as_interaction: true,
            })
            .expect("queue activity");
''',
    '''        handle
            .queue_activity(
                ActivityRecord {
                    event_type: "pet_petted".to_owned(),
                    category: "relationship".to_owned(),
                    relationship_kind: Some("affection".to_owned()),
                    bond_delta: 0.01,
                    counts_as_interaction: true,
                },
                MemoryDecision::Discard,
            )
            .expect("queue activity");
''',
)
insert_before(
    "src-tauri/src/persistence.rs",
    "    #[test]\n    fn journal_ignores_high_frequency_events() {\n",
    '''    #[test]
    fn automatic_memory_is_stored_then_merged() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("bootstrap");
        let mut connection = bootstrap.connection;

        for _ in 0..2 {
            let event = DomainEvent::PetPetted;
            let activity = ActivityRecord::from_domain_event(&event).expect("relationship activity");
            let journal_id = record_activity(&mut connection, &activity).expect("record activity");
            apply_memory_decision(&mut connection, journal_id, memory_evaluator::evaluate(&event))
                .expect("apply memory decision");
        }

        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM memories WHERE kind = 'relationship' AND content = 'The user shows affection by petting Lenvu.'",
                [],
                |row| row.get(0),
            )
            .expect("memory count");
        assert_eq!(count, 1);

        let source_event_id: Option<i64> = connection
            .query_row(
                "SELECT source_event_id FROM memories WHERE content = 'The user shows affection by petting Lenvu.'",
                [],
                |row| row.get(0),
            )
            .expect("memory source");
        assert_eq!(source_event_id, Some(2));
    }

    #[test]
    fn vacuum_into_creates_consistent_backup_file() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("bootstrap");
        let connection = bootstrap.connection;
        let path = std::env::temp_dir().join(format!(
            "northpalace-lenvu-backup-test-{}-{}.sqlite3",
            std::process::id(),
            now_ms()
        ));
        let _ = fs::remove_file(&path);

        backup_database(&connection, &path).expect("backup");
        let backup = Connection::open(&path).expect("open backup");
        let version: i64 = backup
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("backup schema version");
        assert_eq!(version, SCHEMA_VERSION);

        drop(backup);
        fs::remove_file(path).expect("remove backup");
    }

''',
)

replace(
    "src-tauri/src/commands.rs",
    "use std::time::Duration;\n",
    "use std::{path::PathBuf, time::{Duration, SystemTime, UNIX_EPOCH}};\n",
)
replace(
    "src-tauri/src/commands.rs",
    "const PERSISTENCE_ADMIN_TIMEOUT: Duration = Duration::from_secs(3);\n",
    "const PERSISTENCE_ADMIN_TIMEOUT: Duration = Duration::from_secs(3);\nconst PERSISTENCE_BACKUP_TIMEOUT: Duration = Duration::from_secs(15);\n",
)
insert_before(
    "src-tauri/src/commands.rs",
    "#[tauri::command]\npub(crate) fn privacy_get",
    '''#[tauri::command]
pub(crate) async fn data_backup_create(
    app: tauri::AppHandle,
    persistence: tauri::State<'_, PersistenceService>,
) -> Result<String, String> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|error| format!("local-data directory is unavailable: {error}"))?;
    let path = data_dir
        .join("backups")
        .join(format!("lenvu-backup-{}.sqlite3", unix_time_ms()));
    let worker_path = path.clone();
    let persistence = persistence.inner().clone();
    run_persistence_admin(move || persistence.backup_to(worker_path, PERSISTENCE_BACKUP_TIMEOUT))
        .await?;
    Ok(path.to_string_lossy().into_owned())
}

fn unix_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

''',
)
replace(
    "src-tauri/src/lib.rs",
    "        commands::worker_status_get,\n",
    "        commands::worker_status_get,\n        commands::data_backup_create,\n",
    expected=2,
)

print("pre-AI persistence finish patch applied")
