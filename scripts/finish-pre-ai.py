from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def save(path: str, text: str) -> None:
    target = ROOT / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(text, encoding="utf-8", newline="\n")


def literal(path: str, old: str, new: str, expected: int = 1) -> None:
    text = load(path)
    count = text.count(old)
    if count != expected:
        raise RuntimeError(f"{path}: expected {expected} match(es), found {count}: {old!r}")
    save(path, text.replace(old, new))


# ---------------------------------------------------------------------------
# Deterministic Memory Evaluator: store / merge / discard without an LLM.
# ---------------------------------------------------------------------------
save(
    "src-tauri/src/memory_evaluator.rs",
    r'''use std::collections::BTreeSet;

use rusqlite::{Connection, params};

use crate::domain::memory::{MemoryDraft, MemoryKind};

const SIMILARITY_MERGE_THRESHOLD: f32 = 0.84;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MemoryEvaluationOutcome {
    Stored { id: i64 },
    Merged { id: i64, importance: f32 },
    Discarded { reason: &'static str },
}

#[derive(Debug)]
struct ExistingMemory {
    id: i64,
    content: String,
    importance: f32,
    source_event_id: Option<i64>,
}

pub(crate) fn candidate_for_activity(
    event_type: &str,
    occurrence_count: i64,
    source_event_id: i64,
) -> Option<MemoryDraft> {
    if occurrence_count < 3 {
        return None;
    }

    let reinforcement = ((occurrence_count - 3).clamp(0, 20) as f32) * 0.015;
    let (kind, content, base_importance) = match event_type {
        "pet_petted" => (
            MemoryKind::Relationship,
            "The user regularly shows affection by petting Lenvu.",
            0.58,
        ),
        "pet_play" => (
            MemoryKind::Preference,
            "The user repeatedly chooses to play with Lenvu.",
            0.56,
        ),
        "focus_started" => (
            MemoryKind::Preference,
            "The user regularly uses Focus Guard and prefers low-noise companionship while focusing.",
            0.64,
        ),
        _ => return None,
    };

    Some(MemoryDraft {
        kind,
        content: content.to_owned(),
        importance: (base_importance + reinforcement).min(0.92),
        source_event_id: Some(source_event_id),
    })
}

pub(crate) fn evaluate_and_apply(
    connection: &Connection,
    draft: &MemoryDraft,
) -> Result<MemoryEvaluationOutcome, String> {
    let content = normalize_whitespace(&draft.content);
    if content.is_empty() {
        return Ok(MemoryEvaluationOutcome::Discarded {
            reason: "empty_content",
        });
    }
    if content.chars().count() > 10_000 {
        return Ok(MemoryEvaluationOutcome::Discarded {
            reason: "content_too_long",
        });
    }
    if !draft.importance.is_finite() {
        return Ok(MemoryEvaluationOutcome::Discarded {
            reason: "invalid_importance",
        });
    }

    let importance = draft.importance.clamp(0.0, 1.0);
    if importance < minimum_importance(draft.kind) {
        return Ok(MemoryEvaluationOutcome::Discarded {
            reason: "below_importance_threshold",
        });
    }

    if let Some(existing) = best_match(connection, draft.kind, &content)? {
        if existing.source_event_id.is_none() {
            return Ok(MemoryEvaluationOutcome::Discarded {
                reason: "manual_memory_has_authority",
            });
        }

        let reinforced = existing.importance.max(importance);
        connection
            .execute(
                "UPDATE memories SET importance = ?1, updated_at_ms = ?2 WHERE id = ?3",
                params![reinforced, now_ms(), existing.id],
            )
            .map_err(|error| format!("failed to merge evaluated memory: {error}"))?;
        return Ok(MemoryEvaluationOutcome::Merged {
            id: existing.id,
            importance: reinforced,
        });
    }

    let now = now_ms();
    connection
        .execute(
            "INSERT INTO memories (kind, content, importance, source_event_id, created_at_ms, updated_at_ms)\n\
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![
                draft.kind.as_str(),
                content,
                importance,
                draft.source_event_id,
                now
            ],
        )
        .map_err(|error| format!("failed to store evaluated memory: {error}"))?;
    Ok(MemoryEvaluationOutcome::Stored {
        id: connection.last_insert_rowid(),
    })
}

fn minimum_importance(kind: MemoryKind) -> f32 {
    match kind {
        MemoryKind::Episodic => 0.55,
        MemoryKind::Semantic => 0.60,
        MemoryKind::Preference => 0.55,
        MemoryKind::Relationship => 0.50,
    }
}

fn best_match(
    connection: &Connection,
    kind: MemoryKind,
    candidate: &str,
) -> Result<Option<ExistingMemory>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id, content, importance, source_event_id\n\
             FROM memories\n\
             WHERE kind = ?1\n\
             ORDER BY updated_at_ms DESC, id DESC\n\
             LIMIT 100",
        )
        .map_err(|error| format!("failed to prepare evaluator memory scan: {error}"))?;
    let rows = statement
        .query_map(params![kind.as_str()], |row| {
            Ok(ExistingMemory {
                id: row.get(0)?,
                content: row.get(1)?,
                importance: row.get(2)?,
                source_event_id: row.get(3)?,
            })
        })
        .map_err(|error| format!("failed to scan evaluator memories: {error}"))?;

    let mut best: Option<(f32, ExistingMemory)> = None;
    for row in rows {
        let record = row.map_err(|error| format!("failed to read evaluator memory row: {error}"))?;
        let score = similarity(candidate, &record.content);
        if score < SIMILARITY_MERGE_THRESHOLD {
            continue;
        }
        if best.as_ref().is_none_or(|(best_score, _)| score > *best_score) {
            best = Some((score, record));
        }
    }
    Ok(best.map(|(_, record)| record))
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalized_compare(value: &str) -> String {
    normalize_whitespace(value).to_lowercase()
}

fn token_set(value: &str) -> BTreeSet<String> {
    normalized_compare(value)
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .collect()
}

fn similarity(left: &str, right: &str) -> f32 {
    let left_normalized = normalized_compare(left);
    let right_normalized = normalized_compare(right);
    if left_normalized == right_normalized {
        return 1.0;
    }

    let left_tokens = token_set(&left_normalized);
    let right_tokens = token_set(&right_normalized);
    if left_tokens.is_empty() || right_tokens.is_empty() {
        return 0.0;
    }
    let intersection = left_tokens.intersection(&right_tokens).count() as f32;
    let union = left_tokens.union(&right_tokens).count() as f32;
    intersection / union
}

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn connection() -> Connection {
        let connection = Connection::open_in_memory().expect("memory evaluator SQLite");
        connection
            .execute_batch(
                "CREATE TABLE activity_journal (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   event_type TEXT NOT NULL,\n\
                   category TEXT NOT NULL,\n\
                   created_at_ms INTEGER NOT NULL\n\
                 );\n\
                 CREATE TABLE memories (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   kind TEXT NOT NULL,\n\
                   content TEXT NOT NULL,\n\
                   importance REAL NOT NULL,\n\
                   source_event_id INTEGER,\n\
                   created_at_ms INTEGER NOT NULL,\n\
                   updated_at_ms INTEGER NOT NULL,\n\
                   last_accessed_at_ms INTEGER\n\
                 );",
            )
            .expect("memory evaluator schema");
        connection
    }

    #[test]
    fn repeated_activity_becomes_a_candidate_only_after_evidence_threshold() {
        assert!(candidate_for_activity("pet_petted", 2, 1).is_none());
        let candidate = candidate_for_activity("pet_petted", 3, 2).expect("candidate");
        assert_eq!(candidate.kind, MemoryKind::Relationship);
        assert!(candidate.importance >= 0.58);
    }

    #[test]
    fn evaluator_stores_then_merges_automatic_duplicate() {
        let connection = connection();
        let first = candidate_for_activity("focus_started", 3, 1).expect("candidate");
        let stored = evaluate_and_apply(&connection, &first).expect("store");
        let MemoryEvaluationOutcome::Stored { id } = stored else {
            panic!("expected stored outcome");
        };

        let reinforced = candidate_for_activity("focus_started", 8, 2).expect("candidate");
        let merged = evaluate_and_apply(&connection, &reinforced).expect("merge");
        assert!(matches!(
            merged,
            MemoryEvaluationOutcome::Merged { id: merged_id, .. } if merged_id == id
        ));
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM memories", [], |row| row.get(0))
            .expect("memory count");
        assert_eq!(count, 1);
    }

    #[test]
    fn automatic_evaluator_never_overwrites_manual_memory() {
        let connection = connection();
        connection
            .execute(
                "INSERT INTO memories (kind, content, importance, source_event_id, created_at_ms, updated_at_ms)\n\
                 VALUES ('preference', 'The user repeatedly chooses to play with Lenvu.', 0.9, NULL, 1, 1)",
                [],
            )
            .expect("manual memory");
        let candidate = candidate_for_activity("pet_play", 8, 2).expect("candidate");
        assert_eq!(
            evaluate_and_apply(&connection, &candidate).expect("evaluate"),
            MemoryEvaluationOutcome::Discarded {
                reason: "manual_memory_has_authority"
            }
        );
    }
}
''',
)

literal("src-tauri/src/lib.rs", "mod memory_admin;\nmod persistence;", "mod memory_admin;\nmod memory_evaluator;\nmod persistence;")

# Persistence worker integration + online SQLite backup.
literal(
    "src-tauri/src/persistence.rs",
    "    path::Path,\n",
    "    path::{Path, PathBuf},\n",
)
literal(
    "src-tauri/src/persistence.rs",
    "    memory_admin::{self, MemoryInput, MemoryRecord},\n",
    "    memory_admin::{self, MemoryInput, MemoryRecord},\n    memory_evaluator,\n",
)
literal(
    "src-tauri/src/persistence.rs",
    '''    SaveAndFlush {\n        state: PetStateV2,\n        ack: mpsc::SyncSender<Result<(), String>>,\n    },\n    RecordActivity(ActivityRecord),\n''',
    '''    SaveAndFlush {\n        state: PetStateV2,\n        ack: mpsc::SyncSender<Result<(), String>>,\n    },\n    BackupDatabase {\n        destination: PathBuf,\n        ack: mpsc::SyncSender<Result<(), String>>,\n    },\n    RecordActivity(ActivityRecord),\n''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''    #[allow(dead_code)]\n    StoreMemory(MemoryDraft),\n''',
    "",
)
literal(
    "src-tauri/src/persistence.rs",
    '''                    PersistenceCommand::RecordActivity(activity) => {\n                        if let Err(error) = record_activity(&mut connection, &activity) {\n                            eprintln!("Lenvu activity journal write failed: {error}");\n                        }\n                    }\n''',
    '''                    PersistenceCommand::RecordActivity(activity) => {\n                        match record_activity(&mut connection, &activity) {\n                            Ok(journal_id) => match activity_occurrence_count(\n                                &connection,\n                                &activity.event_type,\n                            ) {\n                                Ok(occurrence_count) => {\n                                    if let Some(candidate) = memory_evaluator::candidate_for_activity(\n                                        &activity.event_type,\n                                        occurrence_count,\n                                        journal_id,\n                                    ) && let Err(error) =\n                                        memory_evaluator::evaluate_and_apply(&connection, &candidate)\n                                    {\n                                        eprintln!("Lenvu memory evaluator failed: {error}");\n                                    }\n                                }\n                                Err(error) => {\n                                    eprintln!("Lenvu memory evidence count failed: {error}");\n                                }\n                            },\n                            Err(error) => {\n                                eprintln!("Lenvu activity journal write failed: {error}");\n                            }\n                        }\n                    }\n''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''                    PersistenceCommand::SaveAndFlush { state, ack } => {\n                        let result = save_pet_state(\n                            &mut connection,\n                            &PersistentPetState::from_runtime(&state),\n                        );\n                        let _ = ack.send(result);\n                    }\n''',
    '''                    PersistenceCommand::SaveAndFlush { state, ack } => {\n                        let result = save_pet_state(\n                            &mut connection,\n                            &PersistentPetState::from_runtime(&state),\n                        );\n                        let _ = ack.send(result);\n                    }\n                    PersistenceCommand::BackupDatabase { destination, ack } => {\n                        let _ = ack.send(backup_database(&connection, &destination));\n                    }\n''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''                    PersistenceCommand::StoreMemory(memory) => {\n                        if let Err(error) = insert_memory(&mut connection, &memory) {\n                            eprintln!("Lenvu memory write failed: {error}");\n                        }\n                    }\n''',
    "",
)
literal(
    "src-tauri/src/persistence.rs",
    '''    #[allow(dead_code)]\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\n        self.tx\n            .send(PersistenceCommand::StoreMemory(memory))\n            .map_err(|_| "persistence worker channel is unavailable".to_owned())\n    }\n\n''',
    "",
    expected=2,
)
literal(
    "src-tauri/src/persistence.rs",
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {\n        let (ack_tx, ack_rx) = mpsc::sync_channel(1);\n        self.tx\n            .send(PersistenceCommand::SaveAndFlush { state, ack: ack_tx })\n            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;\n        wait_for_ack(ack_rx, timeout, "persistence final-save")\n    }\n''',
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {\n        let (ack_tx, ack_rx) = mpsc::sync_channel(1);\n        self.tx\n            .send(PersistenceCommand::SaveAndFlush { state, ack: ack_tx })\n            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;\n        wait_for_ack(ack_rx, timeout, "persistence final-save")\n    }\n\n    pub fn backup_to(&self, destination: PathBuf, timeout: Duration) -> Result<(), String> {\n        let (ack_tx, ack_rx) = mpsc::sync_channel(1);\n        self.tx\n            .send(PersistenceCommand::BackupDatabase {\n                destination,\n                ack: ack_tx,\n            })\n            .map_err(|_| "persistence worker channel is unavailable".to_owned())?;\n        wait_for_ack(ack_rx, timeout, "persistence backup")\n    }\n''',
    expected=1,
)
literal(
    "src-tauri/src/persistence.rs",
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {\n        let Some(handle) = self.handle() else {\n            return Ok(());\n        };\n        handle.save_and_flush(state, timeout)\n    }\n''',
    '''    pub fn save_and_flush(&self, state: PetStateV2, timeout: Duration) -> Result<(), String> {\n        let Some(handle) = self.handle() else {\n            return Ok(());\n        };\n        handle.save_and_flush(state, timeout)\n    }\n\n    pub fn backup_to(&self, destination: PathBuf, timeout: Duration) -> Result<(), String> {\n        self.required_handle("persistent local data is unavailable for this session")?\n            .backup_to(destination, timeout)\n    }\n''',
    expected=1,
)
literal(
    "src-tauri/src/persistence.rs",
    "fn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<(), String> {",
    "fn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<i64, String> {",
)
literal(
    "src-tauri/src/persistence.rs",
    '''    transaction\n        .commit()\n        .map_err(|error| format!("failed to commit activity transaction: {error}"))\n}\n\nfn observe_hour(\n''',
    '''    transaction\n        .commit()\n        .map_err(|error| format!("failed to commit activity transaction: {error}"))?;\n    Ok(journal_id)\n}\n\nfn activity_occurrence_count(connection: &Connection, event_type: &str) -> Result<i64, String> {\n    connection\n        .query_row(\n            "SELECT COUNT(*) FROM activity_journal WHERE event_type = ?1",\n            params![event_type],\n            |row| row.get(0),\n        )\n        .map_err(|error| format!("failed to count activity evidence: {error}"))\n}\n\nfn observe_hour(\n''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''fn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<i64, String> {''',
    '''fn backup_database(connection: &Connection, destination: &Path) -> Result<(), String> {\n    if destination.exists() {\n        return Err(format!("backup destination already exists: {}", destination.display()));\n    }\n    if let Some(parent) = destination.parent() {\n        fs::create_dir_all(parent).map_err(|error| {\n            format!("failed to create backup directory {}: {error}", parent.display())\n        })?;\n    }\n    let destination = destination.to_string_lossy().into_owned();\n    connection\n        .execute("VACUUM INTO ?1", params![destination])\n        .map(|_| ())\n        .map_err(|error| format!("failed to create SQLite backup: {error}"))\n}\n\nfn record_activity(connection: &mut Connection, activity: &ActivityRecord) -> Result<i64, String> {''',
)

# Backup regression test uses the same DB worker instead of opening a competing app connection.
literal(
    "src-tauri/src/persistence.rs",
    '''    #[test]\n    fn journal_ignores_high_frequency_events() {\n''',
    '''    #[test]\n    fn online_backup_is_serialized_through_persistence_worker() {\n        let connection = Connection::open_in_memory().expect("in-memory SQLite");\n        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("bootstrap");\n        let supervisor = WorkerSupervisor::default();\n        let handle = bootstrap\n            .into_worker(&supervisor)\n            .expect("persistence worker");\n\n        let nonce = SystemTime::now()\n            .duration_since(UNIX_EPOCH)\n            .unwrap_or_default()\n            .as_nanos();\n        let destination = std::env::temp_dir().join(format!(\n            "northpalace-lenvu-backup-{}-{nonce}.sqlite3",\n            std::process::id()\n        ));\n        let _ = fs::remove_file(&destination);\n        handle\n            .backup_to(destination.clone(), Duration::from_secs(2))\n            .expect("online backup");\n\n        let backup = Connection::open(&destination).expect("open backup");\n        let version: i64 = backup\n            .query_row("PRAGMA user_version", [], |row| row.get(0))\n            .expect("backup schema version");\n        assert_eq!(version, SCHEMA_VERSION);\n        drop(backup);\n        let _ = fs::remove_file(&destination);\n\n        let report = supervisor.shutdown_and_join(Duration::from_secs(1));\n        assert_eq!(report.joined, vec!["persistence-db".to_owned()]);\n        assert!(report.detached.is_empty());\n    }\n\n    #[test]\n    fn journal_ignores_high_frequency_events() {\n''',
)

# ---------------------------------------------------------------------------
# Commands: CSP/IPC production smoke marker + persistence-owner backup.
# ---------------------------------------------------------------------------
literal(
    "src-tauri/src/commands.rs",
    "use std::time::Duration;",
    "use std::time::{Duration, SystemTime, UNIX_EPOCH};",
)
literal(
    "src-tauri/src/commands.rs",
    '''#[derive(Debug, Clone, Serialize)]\n#[serde(rename_all = "camelCase")]\npub(crate) struct StartupStatus {\n    supported: bool,\n    enabled: bool,\n}\n''',
    '''#[derive(Debug, Clone, Serialize)]\n#[serde(rename_all = "camelCase")]\npub(crate) struct StartupStatus {\n    supported: bool,\n    enabled: bool,\n}\n\n#[derive(Debug, Clone, Serialize)]\n#[serde(rename_all = "camelCase")]\npub(crate) struct LocalDataBackupStatus {\n    path: String,\n    created_at_ms: i64,\n}\n''',
)
literal(
    "src-tauri/src/commands.rs",
    '''#[tauri::command]\npub(crate) fn worker_status_get(\n    supervisor: tauri::State<'_, WorkerSupervisor>,\n) -> Vec<WorkerStatus> {\n    supervisor.snapshot()\n}\n''',
    '''#[tauri::command]\npub(crate) fn worker_status_get(\n    supervisor: tauri::State<'_, WorkerSupervisor>,\n) -> Vec<WorkerStatus> {\n    supervisor.snapshot()\n}\n\n#[tauri::command]\npub(crate) fn smoke_probe_mark(window_label: String) -> Result<(), String> {\n    let Ok(marker_path) = std::env::var("NORTHPALACE_SMOKE_MARKER_PATH") else {\n        return Ok(());\n    };\n    if !matches!(window_label.as_str(), "pet" | "companion") {\n        return Err(format!("unexpected smoke-probe window label: {window_label}"));\n    }\n\n    use std::io::Write as _;\n    let mut marker = std::fs::OpenOptions::new()\n        .create(true)\n        .append(true)\n        .open(&marker_path)\n        .map_err(|error| format!("failed to open smoke marker {marker_path}: {error}"))?;\n    writeln!(marker, "{window_label}")\n        .map_err(|error| format!("failed to write smoke marker: {error}"))?;\n    marker\n        .flush()\n        .map_err(|error| format!("failed to flush smoke marker: {error}"))\n}\n''',
)
literal(
    "src-tauri/src/commands.rs",
    '''#[tauri::command]\npub(crate) async fn activity_get(\n    id: i64,\n    persistence: tauri::State<'_, PersistenceService>,\n) -> Result<Option<ActivityHistoryRecord>, String> {\n    let persistence = persistence.inner().clone();\n    run_persistence_admin(move || persistence.get_activity(id, PERSISTENCE_ADMIN_TIMEOUT)).await\n}\n''',
    '''#[tauri::command]\npub(crate) async fn activity_get(\n    id: i64,\n    persistence: tauri::State<'_, PersistenceService>,\n) -> Result<Option<ActivityHistoryRecord>, String> {\n    let persistence = persistence.inner().clone();\n    run_persistence_admin(move || persistence.get_activity(id, PERSISTENCE_ADMIN_TIMEOUT)).await\n}\n\n#[tauri::command]\npub(crate) async fn local_data_backup(\n    app: tauri::AppHandle,\n    persistence: tauri::State<'_, PersistenceService>,\n) -> Result<LocalDataBackupStatus, String> {\n    let created_at_ms = SystemTime::now()\n        .duration_since(UNIX_EPOCH)\n        .unwrap_or_default()\n        .as_millis()\n        .min(i64::MAX as u128) as i64;\n    let destination = app\n        .path()\n        .app_local_data_dir()\n        .map_err(|error| format!("local-data directory is unavailable: {error}"))?\n        .join("backups")\n        .join(format!("lenvu-backup-{created_at_ms}.sqlite3"));\n    let path = destination.display().to_string();\n    let persistence = persistence.inner().clone();\n    run_persistence_admin(move || {\n        persistence.backup_to(destination, PERSISTENCE_ADMIN_TIMEOUT)\n    })\n    .await?;\n    Ok(LocalDataBackupStatus { path, created_at_ms })\n}\n''',
)

# Invoke registration on both platform branches.
literal(
    "src-tauri/src/lib.rs",
    "        commands::worker_status_get,\n        commands::memory_list,",
    "        commands::worker_status_get,\n        commands::smoke_probe_mark,\n        commands::memory_list,",
    expected=2,
)
literal(
    "src-tauri/src/lib.rs",
    "        commands::activity_get,\n        commands::privacy_get,",
    "        commands::activity_get,\n        commands::local_data_backup,\n        commands::privacy_get,",
    expected=2,
)

# Frontend calls the marker once per WebView. Normal runs are a no-op in Rust when env is absent.
literal(
    "src/App.svelte",
    '''  import { getCurrentWindow } from '@tauri-apps/api/window';\n''',
    '''  import { invoke } from '@tauri-apps/api/core';\n  import { getCurrentWindow } from '@tauri-apps/api/window';\n  import { onMount } from 'svelte';\n''',
)
literal(
    "src/App.svelte",
    '''  const windowLabel = getCurrentWindow().label;\n''',
    '''  const windowLabel = getCurrentWindow().label;\n\n  onMount(() => {\n    void invoke('smoke_probe_mark', { windowLabel }).catch(() => undefined);\n  });\n''',
)

# ---------------------------------------------------------------------------
# Deep Settings diagnostics and local-data backup surface.
# ---------------------------------------------------------------------------
literal(
    "src/lib/settings/runtime.ts",
    '''export interface PrivacyRulesSnapshot {\n  excludedApps: string[];\n  accessibilityContextEnabled: boolean;\n  failClosed: boolean;\n}\n''',
    '''export interface PrivacyRulesSnapshot {\n  excludedApps: string[];\n  accessibilityContextEnabled: boolean;\n  failClosed: boolean;\n}\n\nexport type WorkerPhase = 'producers' | 'runtime' | 'journal' | 'persistence';\nexport type WorkerHealth = 'starting' | 'running' | 'stopped' | 'error' | 'panicked' | 'detached';\n\nexport interface WorkerStatus {\n  name: string;\n  phase: WorkerPhase;\n  health: WorkerHealth;\n  lastError: string | null;\n}\n\nexport interface LocalDataBackupStatus {\n  path: string;\n  createdAtMs: number;\n}\n''',
)
literal(
    "src/lib/settings/runtime.ts",
    '''export async function getStartupStatus(): Promise<StartupStatus> {\n  return invoke<StartupStatus>('startup_get');\n}\n''',
    '''export async function getStartupStatus(): Promise<StartupStatus> {\n  return invoke<StartupStatus>('startup_get');\n}\n\nexport async function getWorkerStatus(): Promise<WorkerStatus[]> {\n  return invoke<WorkerStatus[]>('worker_status_get');\n}\n\nexport async function backupLocalData(): Promise<LocalDataBackupStatus> {\n  return invoke<LocalDataBackupStatus>('local_data_backup');\n}\n''',
)

save(
    "src/lib/ui/companion/WorkerDiagnostics.svelte",
    '''<script lang="ts">\n  import { onMount } from 'svelte';\n  import { getWorkerStatus, type WorkerStatus } from '../../settings/runtime';\n\n  let workers: WorkerStatus[] = [];\n  let busy = false;\n  let message = '';\n\n  async function refresh() {\n    busy = true;\n    message = '';\n    try {\n      workers = await getWorkerStatus();\n    } catch (error) {\n      message = error instanceof Error ? error.message : String(error);\n    } finally {\n      busy = false;\n    }\n  }\n\n  onMount(() => {\n    void refresh();\n  });\n</script>\n\n<article class="setting-card">\n  <div class="section-heading">\n    <div class="setting-copy">\n      <strong>Worker diagnostics</strong>\n      <span>深層診斷，只顯示本機 worker phase / health / error；不進入桌寵 ambient surface。</span>\n    </div>\n    <button onclick={() => void refresh()} disabled={busy}>{busy ? '讀取中' : '重新讀取'}</button>\n  </div>\n\n  {#if message}\n    <p class="memory-status">{message}</p>\n  {/if}\n\n  {#if workers.length === 0}\n    <div class="memory-empty">尚未取得 worker 狀態。</div>\n  {:else}\n    <div class="activity-list">\n      {#each workers as worker (worker.name)}\n        <div class="activity-row">\n          <div>\n            <strong>{worker.name}</strong>\n            <span>{worker.phase}</span>\n          </div>\n          <div class="activity-meta">\n            <b>{worker.health}</b>\n            {#if worker.lastError}\n              <span>{worker.lastError}</span>\n            {/if}\n          </div>\n        </div>\n      {/each}\n    </div>\n  {/if}\n</article>\n''',
)

save(
    "src/lib/ui/companion/LocalDataSection.svelte",
    '''<script lang="ts">\n  import { backupLocalData } from '../../settings/runtime';\n\n  let busy = false;\n  let message = '';\n\n  async function backup() {\n    if (busy) return;\n    busy = true;\n    message = '';\n    try {\n      const result = await backupLocalData();\n      message = `SQLite 備份完成：${result.path}`;\n    } catch (error) {\n      message = error instanceof Error ? error.message : String(error);\n    } finally {\n      busy = false;\n    }\n  }\n</script>\n\n<article class="setting-card">\n  <div class="setting-copy">\n    <strong>Local data backup</strong>\n    <span>由唯一 persistence DB owner 執行 SQLite VACUUM INTO 快照，不另外開第二條應用資料庫連線。</span>\n  </div>\n  <button onclick={() => void backup()} disabled={busy}>{busy ? '備份中……' : '建立本機備份'}</button>\n  {#if message}\n    <p class="memory-status">{message}</p>\n  {/if}\n  <div class="settings-note">\n    備份寫入應用 LocalAppData 的 backups 目錄。JSON 可攜匯出與 destructive reset 尚未開放，避免在沒有完整 restore/reset 交易邊界前提供危險按鈕。\n  </div>\n</article>\n''',
)

literal(
    "src/lib/ui/companion/SettingsSection.svelte",
    "  import PrivacySettings from '../PrivacySettings.svelte';\n",
    "  import PrivacySettings from '../PrivacySettings.svelte';\n  import LocalDataSection from './LocalDataSection.svelte';\n  import WorkerDiagnostics from './WorkerDiagnostics.svelte';\n",
)
literal(
    "src/lib/ui/companion/SettingsSection.svelte",
    "    <PrivacySettings />\n",
    "    <PrivacySettings />\n    <LocalDataSection />\n    <WorkerDiagnostics />\n",
)

# ---------------------------------------------------------------------------
# Repository secret guard.
# ---------------------------------------------------------------------------
save(
    "scripts/validate-source-secrets.mjs",
    r'''import { readFileSync, statSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { extname } from 'node:path';

const git = spawnSync('git', ['ls-files', '-z'], { encoding: 'utf8', windowsHide: true });
if (git.error || git.status !== 0) {
  console.error(`Secret guard could not enumerate tracked files: ${git.error?.message ?? git.stderr}`);
  process.exit(1);
}

const binaryExtensions = new Set([
  '.png', '.jpg', '.jpeg', '.webp', '.gif', '.ico', '.zip', '.exe', '.dll', '.woff', '.woff2',
]);
const rules = [
  ['private key block', /-----BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----/g],
  ['GitHub classic token', /\bgh[pousr]_[A-Za-z0-9]{30,}\b/g],
  ['GitHub fine-grained token', /\bgithub_pat_[A-Za-z0-9_]{30,}\b/g],
  ['OpenAI-style secret key', /\bsk-[A-Za-z0-9_-]{24,}\b/g],
  ['AWS access key id', /\bAKIA[A-Z0-9]{16}\b/g],
  ['Slack token', /\bxox[baprs]-[A-Za-z0-9-]{20,}\b/g],
];

const findings = [];
for (const file of git.stdout.split('\0').filter(Boolean)) {
  if (binaryExtensions.has(extname(file).toLowerCase())) continue;
  let stat;
  try { stat = statSync(file); } catch { continue; }
  if (stat.size > 2_000_000) continue;

  let text;
  try { text = readFileSync(file, 'utf8'); } catch { continue; }
  if (text.includes('\0')) continue;

  for (const [name, pattern] of rules) {
    pattern.lastIndex = 0;
    const match = pattern.exec(text);
    if (match) findings.push({ file: file.replaceAll('\\', '/'), rule: name, index: match.index });
  }
}

if (findings.length > 0) {
  console.error('Secret guard found credential-like material in tracked text:');
  for (const finding of findings) console.error(`- ${finding.file}: ${finding.rule} @ ${finding.index}`);
  process.exit(1);
}
console.log('Secret guard passed: no common credential/private-key patterns found in tracked text.');
''',
)

package = json.loads(load("package.json"))
package["scripts"]["validate:secrets"] = "node scripts/validate-source-secrets.mjs"
package["scripts"]["measure:target"] = "powershell -NoProfile -ExecutionPolicy Bypass -File scripts/measure-target-baseline.ps1"
save("package.json", json.dumps(package, ensure_ascii=False, indent=2) + "\n")

literal(
    ".github/workflows/windows-ci.yml",
    "      - name: Reject tracked local/private runtime data\n        run: npm run validate:repo\n\n",
    "      - name: Reject tracked local/private runtime data\n        run: npm run validate:repo\n\n      - name: Reject tracked credential-like source material\n        run: npm run validate:secrets\n\n",
)

# Permanent workflow: source gates + production WebView/Tauri IPC smoke + artifact upload.
save(
    ".github/workflows/windows-bundle.yml",
    r'''name: Windows Bundle

on:
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: windows-bundle-${{ github.ref }}
  cancel-in-progress: true

jobs:
  bundle:
    runs-on: windows-latest
    timeout-minutes: 45

    steps:
      - name: Checkout
        uses: actions/checkout@v6

      - name: Setup Node.js
        uses: actions/setup-node@v6
        with:
          node-version: 24
          package-manager-cache: false

      - name: Reject tracked local/private runtime data
        run: npm run validate:repo

      - name: Reject tracked credential-like source material
        run: npm run validate:secrets

      - name: Install locked frontend dependencies
        run: npm ci --no-audit --no-fund

      - name: Check Svelte with TypeScript 6
        run: npm run check:svelte

      - name: Check Svelte with TypeScript 7 tsgo
        run: npm run check:svelte:tsgo

      - name: Install stable Rust
        shell: pwsh
        run: |
          rustup toolchain install stable --profile minimal
          rustup default stable

      - name: Verify committed Cargo lockfile
        run: cargo metadata --locked --manifest-path src-tauri/Cargo.toml --format-version 1 > NUL

      - name: Build Windows executable and NSIS bundle
        run: npm run desktop:build -- --bundles nsis

      - name: Verify production CSP and Tauri IPC in both WebViews
        shell: pwsh
        run: |
          $marker = Join-Path $env:RUNNER_TEMP "lenvu-production-smoke.txt"
          Remove-Item $marker -Force -ErrorAction SilentlyContinue
          $app = Get-ChildItem src-tauri/target/release -Filter northpalace-my-pet.exe -File | Select-Object -First 1
          if (-not $app) { throw "release executable not found" }

          $env:NORTHPALACE_SMOKE_MARKER_PATH = $marker
          $process = $null
          try {
            $process = Start-Process -FilePath $app.FullName -PassThru
            $deadline = (Get-Date).AddSeconds(20)
            $ready = $false
            do {
              Start-Sleep -Milliseconds 500
              if ($process.HasExited) {
                throw "release executable exited during CSP/IPC smoke with code $($process.ExitCode)"
              }
              $labels = if (Test-Path $marker) { @(Get-Content $marker) } else { @() }
              $ready = ($labels -contains "pet") -and ($labels -contains "companion")
            } while (-not $ready -and (Get-Date) -lt $deadline)

            if (-not $ready) {
              $observed = if (Test-Path $marker) { (Get-Content $marker) -join ", " } else { "none" }
              throw "production WebView/Tauri IPC smoke timed out; observed labels: $observed"
            }
          }
          finally {
            Remove-Item Env:NORTHPALACE_SMOKE_MARKER_PATH -ErrorAction SilentlyContinue
            if ($process -and -not $process.HasExited) {
              Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
              Wait-Process -Id $process.Id -ErrorAction SilentlyContinue
            }
          }

      - name: Upload Windows bundle
        uses: actions/upload-artifact@v7
        with:
          name: northpalace-windows-${{ github.sha }}
          path: |
            src-tauri/target/release/*.exe
            src-tauri/target/release/bundle/nsis/**
          if-no-files-found: error
          retention-days: 7
''',
)

# ---------------------------------------------------------------------------
# Target-machine measurement harness. No global package/tool installation.
# ---------------------------------------------------------------------------
save(
    "scripts/measure-target-baseline.ps1",
    r'''param(
  [string]$ExecutablePath = "src-tauri/target/release/northpalace-my-pet.exe",
  [string]$Scenario = "idle",
  [int]$WarmupSeconds = 10,
  [int]$SampleSeconds = 60,
  [int]$IntervalMs = 1000,
  [switch]$Launch,
  [string]$OutputDirectory = ".workspace/benchmarks"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Get-DescendantIds([int]$RootId) {
  $processRows = @(Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId)
  $ids = [System.Collections.Generic.HashSet[int]]::new()
  [void]$ids.Add($RootId)
  $changed = $true
  while ($changed) {
    $changed = $false
    foreach ($row in $processRows) {
      if ($ids.Contains([int]$row.ParentProcessId) -and -not $ids.Contains([int]$row.ProcessId)) {
        [void]$ids.Add([int]$row.ProcessId)
        $changed = $true
      }
    }
  }
  return @($ids)
}

function Get-GpuSample([int[]]$Ids) {
  try {
    $sample = Get-Counter '\GPU Engine(*)\Utilization Percentage' -ErrorAction Stop
    $values = @()
    foreach ($counter in $sample.CounterSamples) {
      if ($counter.Path -match 'pid_(\d+)_') {
        $counterPid = [int]$Matches[1]
        if ($Ids -contains $counterPid) { $values += [double]$counter.CookedValue }
      }
    }
    if ($values.Count -eq 0) { return @{ total = $null; maxEngine = $null } }
    return @{ total = ($values | Measure-Object -Sum).Sum; maxEngine = ($values | Measure-Object -Maximum).Maximum }
  }
  catch {
    return @{ total = $null; maxEngine = $null }
  }
}

function Percentile([double[]]$Values, [double]$P) {
  if (-not $Values -or $Values.Count -eq 0) { return $null }
  $sorted = @($Values | Sort-Object)
  $index = [Math]::Min($sorted.Count - 1, [Math]::Max(0, [Math]::Ceiling($P * $sorted.Count) - 1))
  return [double]$sorted[$index]
}

$resolved = [System.IO.Path]::GetFullPath((Join-Path (Get-Location) $ExecutablePath))
if (-not (Test-Path $resolved)) { throw "Executable not found: $resolved" }
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null

$ownedProcess = $false
$process = $null
if ($Launch) {
  $process = Start-Process -FilePath $resolved -PassThru
  $ownedProcess = $true
} else {
  $process = Get-Process -Name 'northpalace-my-pet' -ErrorAction SilentlyContinue | Select-Object -First 1
  if (-not $process) { throw "northpalace-my-pet is not running; use -Launch or start it first" }
}

try {
  if ($WarmupSeconds -gt 0) { Start-Sleep -Seconds $WarmupSeconds }
  if ($process.HasExited) { throw "Lenvu exited during warmup with code $($process.ExitCode)" }

  $logicalProcessors = [Math]::Max(1, [Environment]::ProcessorCount)
  $samples = @()
  $previousCpu = $null
  $previousTime = $null
  $deadline = (Get-Date).AddSeconds([Math]::Max(1, $SampleSeconds))

  while ((Get-Date) -lt $deadline) {
    if ($process.HasExited) { throw "Lenvu exited during sampling with code $($process.ExitCode)" }
    $ids = @(Get-DescendantIds $process.Id)
    $members = @(Get-Process -Id $ids -ErrorAction SilentlyContinue)
    $now = Get-Date
    $cpuSeconds = ($members | Measure-Object -Property CPU -Sum).Sum
    if ($null -eq $cpuSeconds) { $cpuSeconds = 0.0 }
    $cpuPercent = $null
    if ($null -ne $previousCpu -and $null -ne $previousTime) {
      $elapsed = ($now - $previousTime).TotalSeconds
      if ($elapsed -gt 0) {
        $cpuPercent = (($cpuSeconds - $previousCpu) / $elapsed / $logicalProcessors) * 100.0
      }
    }
    $previousCpu = [double]$cpuSeconds
    $previousTime = $now

    $workingSet = ($members | Measure-Object -Property WorkingSet64 -Sum).Sum
    $privateBytes = ($members | Measure-Object -Property PrivateMemorySize64 -Sum).Sum
    $gpu = Get-GpuSample $ids
    $samples += [pscustomobject]@{
      timestamp = $now.ToUniversalTime().ToString('o')
      processCount = $members.Count
      cpuPercent = if ($null -eq $cpuPercent) { $null } else { [Math]::Max(0.0, $cpuPercent) }
      workingSetBytes = [int64]$workingSet
      privateBytes = [int64]$privateBytes
      gpuEngineTotalPercent = $gpu.total
      gpuMaxEnginePercent = $gpu.maxEngine
    }
    Start-Sleep -Milliseconds ([Math]::Max(250, $IntervalMs))
  }

  $os = Get-CimInstance Win32_OperatingSystem
  $cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
  $gpus = @(Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name)
  $hash = (Get-FileHash $resolved -Algorithm SHA256).Hash
  $cpuValues = @($samples | Where-Object { $null -ne $_.cpuPercent } | ForEach-Object { [double]$_.cpuPercent })
  $workingValues = @($samples | ForEach-Object { [double]$_.workingSetBytes })
  $privateValues = @($samples | ForEach-Object { [double]$_.privateBytes })
  $gpuValues = @($samples | Where-Object { $null -ne $_.gpuEngineTotalPercent } | ForEach-Object { [double]$_.gpuEngineTotalPercent })

  $summary = [ordered]@{
    cpuAveragePercent = if ($cpuValues.Count) { ($cpuValues | Measure-Object -Average).Average } else { $null }
    cpuP95Percent = Percentile $cpuValues 0.95
    cpuMaxPercent = if ($cpuValues.Count) { ($cpuValues | Measure-Object -Maximum).Maximum } else { $null }
    workingSetAverageBytes = if ($workingValues.Count) { [int64](($workingValues | Measure-Object -Average).Average) } else { $null }
    workingSetP95Bytes = if ($workingValues.Count) { [int64](Percentile $workingValues 0.95) } else { $null }
    privateAverageBytes = if ($privateValues.Count) { [int64](($privateValues | Measure-Object -Average).Average) } else { $null }
    gpuAverageEngineTotalPercent = if ($gpuValues.Count) { ($gpuValues | Measure-Object -Average).Average } else { $null }
    gpuP95EngineTotalPercent = Percentile $gpuValues 0.95
  }

  $result = [ordered]@{
    schemaVersion = 1
    scenario = $Scenario
    recordedAtUtc = (Get-Date).ToUniversalTime().ToString('o')
    executable = [ordered]@{ path = $resolved; sha256 = $hash }
    system = [ordered]@{
      os = $os.Caption
      osVersion = $os.Version
      cpu = $cpu.Name
      logicalProcessors = $logicalProcessors
      totalVisibleMemoryBytes = [int64]$os.TotalVisibleMemorySize * 1024
      gpu = $gpus
    }
    sampling = [ordered]@{
      warmupSeconds = $WarmupSeconds
      sampleSeconds = $SampleSeconds
      intervalMs = $IntervalMs
      sampleCount = $samples.Count
    }
    summary = $summary
    samples = $samples
  }

  $stamp = (Get-Date).ToString('yyyyMMdd-HHmmss')
  $safeScenario = ($Scenario -replace '[^A-Za-z0-9_-]', '_')
  $base = Join-Path $OutputDirectory "lenvu-$safeScenario-$stamp"
  $jsonPath = "$base.json"
  $mdPath = "$base.md"
  $result | ConvertTo-Json -Depth 8 | Set-Content $jsonPath -Encoding utf8

  $mb = 1MB
  $markdown = @(
    '# Lenvu target-machine baseline',
    '',
    "- Scenario: $Scenario",
    "- Recorded UTC: $($result.recordedAtUtc)",
    "- Executable SHA-256: $hash",
    "- OS: $($os.Caption) $($os.Version)",
    "- CPU: $($cpu.Name)",
    "- GPU: $($gpus -join ', ')",
    "- Total visible RAM: $([Math]::Round(($result.system.totalVisibleMemoryBytes / $mb), 1)) MB",
    '',
    '| Metric | Result |',
    '|---|---:|',
    "| CPU average | $([Math]::Round([double]($summary.cpuAveragePercent ?? 0), 2)) % |",
    "| CPU p95 | $([Math]::Round([double]($summary.cpuP95Percent ?? 0), 2)) % |",
    "| CPU max | $([Math]::Round([double]($summary.cpuMaxPercent ?? 0), 2)) % |",
    "| Working set average | $([Math]::Round(([double]($summary.workingSetAverageBytes ?? 0) / $mb), 1)) MB |",
    "| Working set p95 | $([Math]::Round(([double]($summary.workingSetP95Bytes ?? 0) / $mb), 1)) MB |",
    "| Private bytes average | $([Math]::Round(([double]($summary.privateAverageBytes ?? 0) / $mb), 1)) MB |",
    "| GPU engine-total average* | $([Math]::Round([double]($summary.gpuAverageEngineTotalPercent ?? 0), 2)) % |",
    "| GPU engine-total p95* | $([Math]::Round([double]($summary.gpuP95EngineTotalPercent ?? 0), 2)) % |",
    '',
    '* GPU counters are best-effort Windows GPU Engine counters and can be unavailable on some drivers. The JSON retains null when unavailable.',
    '',
    "Raw samples: `$jsonPath`"
  )
  $markdown | Set-Content $mdPath -Encoding utf8
  Write-Host "Wrote $jsonPath"
  Write-Host "Wrote $mdPath"
}
finally {
  if ($ownedProcess -and $process -and -not $process.HasExited) {
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    Wait-Process -Id $process.Id -ErrorAction SilentlyContinue
  }
}
''',
)

save(
    "docs/TARGET_MACHINE_VALIDATION.md",
    '''# Target Windows Validation\n\nThe final hardware gate belongs to the actual **Windows 11 / Ryzen 3 2200G / 16 GB / Vega 8** machine. GitHub-hosted Windows proves buildability, not idle cost or iGPU behavior.\n\n## Measurement command\n\nBuild or install the release executable, then from the repository workspace run:\n\n```powershell\nnpm run measure:target -- -Launch -Scenario idle -WarmupSeconds 15 -SampleSeconds 120\n```\n\nThe harness uses only built-in Windows / PowerShell facilities. It records the NorthPalace process plus descendant WebView2 processes, CPU, working/private memory and best-effort Windows GPU Engine counters. Output is written under `.workspace/benchmarks/`, which is intentionally untracked.\n\n## Required scenarios\n\nRun at least these scenarios on the target machine:\n\n1. `idle` — Lenvu visible, Companion hidden, no interaction.\n2. `companion-open` — Companion open on Home/Memory for ordinary management load.\n3. `motion-interaction` — walking/explore plus pet/drag interactions.\n4. `long-idle` — at least 10 minutes separately to observe sleep/rest stability and memory growth.\n5. `multi-monitor` when a second display is available — verify work-area boundaries and horizontal Explore transition.\n\nFor scenarios where you prepare the UI state manually, start Lenvu first and omit `-Launch`; the script attaches to the existing `northpalace-my-pet` process.\n\n## Acceptance evidence\n\nDo not invent fixed performance numbers before the target machine is measured. Keep the generated JSON + Markdown together and compare regressions by executable SHA-256. The gate closes only after the actual R3 2200G report shows acceptable idle CPU/RAM/GPU behavior and no crash/stall under the scenarios above.\n''',
)

save(
    "docs/LOCAL_DATA_POLICY.md",
    '''# Local Data Backup / Export / Reset Policy\n\nLenvu local life data is user-owned and local-first. The application must not create a second competing SQLite owner just to implement management actions.\n\n## Implemented now: online SQLite backup\n\nSettings can request a backup through the existing `persistence-db` worker. The worker executes SQLite `VACUUM INTO` and writes a timestamped `lenvu-backup-*.sqlite3` under the app LocalAppData `backups` directory. This keeps backup ordering inside the same serialized database owner used by pet state, journal and memories.\n\n## Portable export\n\nA future portable JSON export should be versioned, human-inspectable and exclude transient UI/runtime state. It must include provenance/schema metadata and must not silently include privacy rules or model files. It is intentionally not exposed until an import/restore compatibility contract exists.\n\n## Destructive reset\n\nReset remains intentionally unavailable. A correct reset has to coordinate Pet Runtime long-lived state, autosave, SQLite tables, relationship/memory history and separately stored privacy rules. Deleting the database file while the worker is live, or clearing tables while autosave can immediately repopulate stale state, is forbidden.\n\nThe UI must not offer a destructive reset button until that coordinated transaction has tests and an explicit confirmation/restore story.\n''',
)

# ---------------------------------------------------------------------------
# Source-derived visual evidence pack: never runtime/production artwork.
# ---------------------------------------------------------------------------
source_href = "../../../../../reference/anatomy/lenvu-anatomy-reference.webp"
for filename, label, region in [
    ("three-quarter-source-evidence-v1.svg", "three-quarter neutral source evidence", (1100, 15, 420, 510)),
    ("head-identity-source-evidence-v1.svg", "head front/profile source evidence", (30, 525, 675, 485)),
    ("lumen-paws-tail-source-evidence-v1.svg", "Lumen-Code / paws / tail source evidence", (705, 550, 810, 460)),
]:
    x, y, width, height = region
    scale = min(960 / width, 900 / height)
    tx = (1024 - width * scale) / 2 - x * scale
    ty = 62 - y * scale
    save(
        f"assets/runtime/lenvu/master/review/source-evidence/{filename}",
        f'''<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024">\n  <defs><clipPath id="crop"><rect x="{x}" y="{y}" width="{width}" height="{height}"/></clipPath></defs>\n  <rect width="1024" height="1024" fill="#0b1420"/>\n  <text x="32" y="38" fill="#9edff0" font-family="sans-serif" font-size="18">Lenvu — {label}</text>\n  <g transform="matrix({scale:.12f} 0 0 {scale:.12f} {tx:.12f} {ty:.12f})" clip-path="url(#crop)">\n    <image href="{source_href}" x="0" y="0" width="1536" height="1024"/>\n  </g>\n  <text x="32" y="998" fill="#6f92a3" font-family="sans-serif" font-size="13">direct crop from primary anatomy authority; review evidence only; no redraw / no generation</text>\n</svg>\n''',
    )

candidate_path = ROOT / "assets/runtime/lenvu/source-notes/master-candidate.json"
candidate = json.loads(candidate_path.read_text(encoding="utf-8"))
staging = candidate.setdefault("sourceDerivedStaging", {})
staging["evidenceViews"] = {
    "threeQuarter": "assets/runtime/lenvu/master/review/source-evidence/three-quarter-source-evidence-v1.svg",
    "headIdentity": "assets/runtime/lenvu/master/review/source-evidence/head-identity-source-evidence-v1.svg",
    "lumenPawsTail": "assets/runtime/lenvu/master/review/source-evidence/lumen-paws-tail-source-evidence-v1.svg",
}
candidate_path.write_text(json.dumps(candidate, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

literal(
    "assets/runtime/lenvu/master/README.md",
    '''The review gate is tracked by `assets/runtime/lenvu/source-notes/master-candidate.json`. Identity asymmetry is non-negotiable: Lenvu right eye is cyan, left eye is violet, and the gold crescent/ring belongs to Lenvu's left horn. Blind horizontal mirroring is forbidden.\n''',
    '''The review gate is tracked by `assets/runtime/lenvu/source-notes/master-candidate.json`. Identity asymmetry is non-negotiable: Lenvu right eye is cyan, left eye is violet, and the gold crescent/ring belongs to Lenvu's left horn. Blind horizontal mirroring is forbidden.\n\n`review/source-normalized/` contains measured 1024×1024 front/profile/back source mappings. `review/source-evidence/` contains direct high-resolution source crops for 3/4, head identity and Lumen-Code/paw/tail QA. These are source-derived review aids, **not** canonical artwork and never runtime textures.\n''',
)

# ---------------------------------------------------------------------------
# Documentation state after this pass (manual workflow/CSP evidence is recorded later).
# ---------------------------------------------------------------------------
literal(
    "docs/ROADMAP.md",
    "- [ ] High-quality source-faithful anatomy/README derivatives.\n",
    "- [x] Source-faithful anatomy/reference derivatives for master QA (direct source-derived normalization/evidence; not production artwork).\n",
)
literal(
    "docs/ROADMAP.md",
    "- [ ] Memory Evaluator: store / merge / discard.\n",
    "- [x] Deterministic Memory Evaluator: evidence-threshold store / merge / discard without an LLM; manual memories retain authority.\n",
)
literal(
    "docs/ROADMAP.md",
    "- [ ] Local-data export/reset/backup policy.\n",
    "- [ ] Local-data export/reset/backup policy (online SQLite backup implemented; portable export + coordinated destructive reset remain pending).\n",
)
literal(
    "docs/ROADMAP.md",
    "- [x] Lock a Svelte-specific TS6 + TypeScript 7 `--tsgo` diagnostic gate with zero warnings (`docs/SVELTE_DIAGNOSTIC_BASELINE.md`).\n",
    "- [x] Lock a Svelte-specific TS6 + TypeScript 7 `--tsgo` diagnostic gate with zero warnings (`docs/SVELTE_DIAGNOSTIC_BASELINE.md`).\n- [x] Expose named worker phase/health/error state in the deep Settings diagnostics surface.\n- [x] Add a no-global-install target-machine CPU/RAM/GPU measurement harness (`scripts/measure-target-baseline.ps1`).\n",
)

literal(
    "README.md",
    "- Memory Browser/editor, Activity and Privacy/Settings surfaces;\n",
    "- Memory Browser/editor, Activity and Privacy/Settings surfaces, including deep worker diagnostics and persistence-owner local backup;\n",
)
literal(
    "README.md",
    "- source-measured Lenvu visual ground-truth / canonical landmark pipeline.\n",
    "- deterministic non-LLM Memory Evaluator for repeated interaction evidence (store / merge / discard while manual memories retain authority);\n- source-measured Lenvu visual ground-truth / canonical landmark pipeline plus direct source-derived normalization/evidence views.\n",
)
literal(
    "README.md",
    "- Memory Evaluator and deeper long-term relationship/personality evolution;\n",
    "- deeper long-term relationship/personality evolution and portable local-data export/coordinated reset;\n",
)
literal(
    "README.md",
    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate and `docs/VALIDATION_BASELINE.md` records the clean frontend/Rust gate. `docs/WINDOWS_BUNDLE_BASELINE.md` records a clean GitHub-hosted Windows release/NSIS build, artifact discovery and bounded release-executable smoke launch. The manual Windows Bundle workflow consumes the same committed dependency graphs rather than generating independent candidates.\n",
    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate and `docs/VALIDATION_BASELINE.md` records the clean frontend/Rust gate. `docs/WINDOWS_BUNDLE_BASELINE.md` records a clean GitHub-hosted Windows release/NSIS build, artifact discovery and bounded release-executable smoke launch. The permanent Windows Bundle workflow consumes the same committed dependency graphs and now also requires both production WebViews to cross Tauri IPC under the production CSP before artifact upload.\n",
)
literal(
    "README.md",
    "canonical Lenvu production master\n  ↓\nIdle / Walk / Sit / Sleep production assets\n",
    "canonical Lenvu production master\n  ↓\nIdle / Walk / Sit / Lie / Sleep / Wake production assets\n",
)

literal(
    "docs/ARCHITECTURE.md",
    "Pet-state saves, autosave, domain-event journaling, hourly rhythm writes, future memory-candidate storage, Memory Browser CRUD/search and Activity History list/get all serialize through the same `PersistenceCommand` queue.",
    "Pet-state saves, autosave, domain-event journaling, hourly rhythm writes, deterministic Memory Evaluator writes, online SQLite backup, Memory Browser CRUD/search and Activity History list/get all serialize through the same `PersistenceCommand` queue.",
)
literal(
    "docs/ARCHITECTURE.md",
    "Memory category and transport contracts have one domain source of truth in `src-tauri/src/domain/memory.rs`.",
    "Repeated relationship/focus activity is evaluated after it has accumulated an evidence threshold. The deterministic evaluator stores a new automatic memory, merges a sufficiently similar automatic memory, or discards low-confidence/duplicate evidence; it never overwrites a manual Memory Browser record. No LLM participates in this path.\n\nOnline SQLite backup also runs through the persistence owner via `VACUUM INTO`; portable JSON export and destructive reset remain withheld until restore/reset coordination is proven.\n\nMemory category and transport contracts have one domain source of truth in `src-tauri/src/domain/memory.rs`.",
)
literal(
    "docs/ARCHITECTURE.md",
    "The supervisor itself is Tauri-managed application state. `worker_status_get` exposes a small structured health snapshot, including each worker's shutdown phase, for future deep-management/debug surfaces without pushing worker administration into the ambient pet UI.",
    "The supervisor itself is Tauri-managed application state. `worker_status_get` exposes a small structured health snapshot, including each worker's shutdown phase, in the deep Settings diagnostics surface without pushing worker administration into the ambient pet UI.",
)

print("Pre-AI finish patch applied.")
