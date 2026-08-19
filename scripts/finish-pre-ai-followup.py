from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def save(path: str, text: str) -> None:
    (ROOT / path).write_text(text, encoding="utf-8", newline="\n")


def literal(path: str, old: str, new: str, expected: int = 1) -> None:
    text = load(path)
    count = text.count(old)
    if count != expected:
        raise RuntimeError(f"{path}: expected {expected} match(es), found {count}: {old!r}")
    save(path, text.replace(old, new))


# The first finish script historically expected two identical PersistenceHandle queue_memory
# methods; only one exists. This follow-up runs after the corrected first script and removes
# the differently-shaped PersistenceService transport method separately.
literal(
    "src-tauri/src/persistence.rs",
    '''    #[allow(dead_code)]
    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
        let Some(handle) = self.handle() else {
            return Ok(());
        };
        handle.queue_memory(memory)
    }

''',
    "",
)

# ---------------------------------------------------------------------------
# Durable memory provenance: automatic/manual origin survives journal retention.
# ---------------------------------------------------------------------------
literal(
    "src-tauri/src/domain/memory.rs",
    '''impl MemoryKind {
    pub fn as_str(self) -> &'static str {
''',
    '''#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryOrigin {
    Manual,
    Automatic,
}

impl MemoryOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Automatic => "automatic",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "manual" => Some(Self::Manual),
            "automatic" => Some(Self::Automatic),
            _ => None,
        }
    }
}

impl MemoryKind {
    pub fn as_str(self) -> &'static str {
''',
)
literal(
    "src-tauri/src/domain/memory.rs",
    '''        assert_eq!(MemoryKind::from_str("unknown"), None);
    }
}
''',
    '''        assert_eq!(MemoryKind::from_str("unknown"), None);
    }

    #[test]
    fn memory_origin_storage_round_trip_is_stable() {
        for (origin, raw) in [
            (MemoryOrigin::Manual, "manual"),
            (MemoryOrigin::Automatic, "automatic"),
        ] {
            assert_eq!(origin.as_str(), raw);
            assert_eq!(MemoryOrigin::from_str(raw), Some(origin));
        }
        assert_eq!(MemoryOrigin::from_str("unknown"), None);
    }
}
''',
)

literal(
    "src-tauri/src/memory_evaluator.rs",
    "use crate::domain::memory::{MemoryDraft, MemoryKind};",
    "use crate::domain::memory::{MemoryDraft, MemoryKind, MemoryOrigin};",
)
literal(
    "src-tauri/src/memory_evaluator.rs",
    '''struct ExistingMemory {
    id: i64,
    content: String,
    importance: f32,
    source_event_id: Option<i64>,
}
''',
    '''struct ExistingMemory {
    id: i64,
    content: String,
    importance: f32,
    origin: MemoryOrigin,
}
''',
)
literal(
    "src-tauri/src/memory_evaluator.rs",
    "        if existing.source_event_id.is_none() {",
    "        if existing.origin == MemoryOrigin::Manual {",
)
literal(
    "src-tauri/src/memory_evaluator.rs",
    '''            "INSERT INTO memories (kind, content, importance, source_event_id, created_at_ms, updated_at_ms)\n\
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
''',
    '''            "INSERT INTO memories (kind, content, importance, source_event_id, origin, created_at_ms, updated_at_ms)\n\
             VALUES (?1, ?2, ?3, ?4, 'automatic', ?5, ?5)",
''',
)
literal(
    "src-tauri/src/memory_evaluator.rs",
    '''            "SELECT id, content, importance, source_event_id\n\
             FROM memories\n\
''',
    '''            "SELECT id, content, importance, origin\n\
             FROM memories\n\
''',
)
literal(
    "src-tauri/src/memory_evaluator.rs",
    '''        .query_map(params![kind.as_str()], |row| {
            Ok(ExistingMemory {
                id: row.get(0)?,
                content: row.get(1)?,
                importance: row.get(2)?,
                source_event_id: row.get(3)?,
            })
        })
''',
    '''        .query_map(params![kind.as_str()], |row| {
            let origin: String = row.get(3)?;
            Ok(ExistingMemory {
                id: row.get(0)?,
                content: row.get(1)?,
                importance: row.get(2)?,
                origin: MemoryOrigin::from_str(&origin).unwrap_or(MemoryOrigin::Manual),
            })
        })
''',
)
literal(
    "src-tauri/src/memory_evaluator.rs",
    '''                   source_event_id INTEGER,
                   created_at_ms INTEGER NOT NULL,
''',
    '''                   source_event_id INTEGER,
                   origin TEXT NOT NULL DEFAULT 'manual',
                   created_at_ms INTEGER NOT NULL,
''',
)

literal(
    "src-tauri/src/memory_admin.rs",
    "pub use crate::domain::memory::MemoryKind;",
    "pub use crate::domain::memory::{MemoryKind, MemoryOrigin};",
)
literal(
    "src-tauri/src/memory_admin.rs",
    '''    pub source_event_id: Option<i64>,
}
''',
    '''    pub source_event_id: Option<i64>,
    pub origin: MemoryOrigin,
}
''',
)
literal(
    "src-tauri/src/memory_admin.rs",
    "SELECT id, kind, content, importance, created_at_ms, updated_at_ms, source_event_id\\n\\\n",
    "SELECT id, kind, content, importance, created_at_ms, updated_at_ms, source_event_id, origin\\n\\\n",
    expected=2,
)
literal(
    "src-tauri/src/memory_admin.rs",
    '''            "SELECT m.id, m.kind, m.content, m.importance, m.created_at_ms,\n\
                    m.updated_at_ms, m.source_event_id\n\
''',
    '''            "SELECT m.id, m.kind, m.content, m.importance, m.created_at_ms,\n\
                    m.updated_at_ms, m.source_event_id, m.origin\n\
''',
)
literal(
    "src-tauri/src/memory_admin.rs",
    '''               kind, content, importance, source_event_id, created_at_ms, updated_at_ms\n\
             ) VALUES (?1, ?2, ?3, NULL, ?4, ?4)",
''',
    '''               kind, content, importance, source_event_id, origin, created_at_ms, updated_at_ms\n\
             ) VALUES (?1, ?2, ?3, NULL, 'manual', ?4, ?4)",
''',
)
literal(
    "src-tauri/src/memory_admin.rs",
    '''    let Some(kind) = MemoryKind::from_str(&kind_raw) else {
        return Ok(None);
    };

    Ok(Some(MemoryRecord {
''',
    '''    let Some(kind) = MemoryKind::from_str(&kind_raw) else {
        return Ok(None);
    };
    let origin_raw: String = row.get(7)?;
    let origin = MemoryOrigin::from_str(&origin_raw).unwrap_or(MemoryOrigin::Manual);

    Ok(Some(MemoryRecord {
''',
)
literal(
    "src-tauri/src/memory_admin.rs",
    '''        source_event_id: row.get(6)?,
    }))
''',
    '''        source_event_id: row.get(6)?,
        origin,
    }))
''',
)
literal(
    "src-tauri/src/memory_admin.rs",
    '''                   source_event_id INTEGER,
                   created_at_ms INTEGER NOT NULL,
''',
    '''                   source_event_id INTEGER,
                   origin TEXT NOT NULL DEFAULT 'manual',
                   created_at_ms INTEGER NOT NULL,
''',
)

literal("src-tauri/src/persistence.rs", "const SCHEMA_VERSION: i64 = 2;", "const SCHEMA_VERSION: i64 = 3;")
literal(
    "src-tauri/src/persistence.rs",
    '''                   source_event_id INTEGER,\n\
                   created_at_ms INTEGER NOT NULL,\n\
''',
    '''                   source_event_id INTEGER,\n\
                   origin TEXT NOT NULL DEFAULT 'manual' CHECK (origin IN ('manual', 'automatic')),\n\
                   created_at_ms INTEGER NOT NULL,\n\
''',
    expected=2,
)
literal(
    "src-tauri/src/persistence.rs",
    "                 PRAGMA user_version = 2;\\n\\\n                 COMMIT;",
    "                 PRAGMA user_version = 3;\\n\\\n                 COMMIT;",
    expected=2,
)
literal(
    "src-tauri/src/persistence.rs",
    '''        SCHEMA_VERSION => Ok(()),
''',
    '''        2 => connection
            .execute_batch(
                "BEGIN IMMEDIATE;\n\
                 ALTER TABLE memories ADD COLUMN origin TEXT NOT NULL DEFAULT 'manual'\n\
                   CHECK (origin IN ('manual', 'automatic'));\n\
                 PRAGMA user_version = 3;\n\
                 COMMIT;",
            )
            .map_err(|error| format!("failed to migrate SQLite schema v2 -> v3: {error}")),
        SCHEMA_VERSION => Ok(()),
''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''fn insert_memory(connection: &mut Connection, memory: &MemoryDraft) -> Result<i64, String> {
''',
    '''#[cfg(test)]
fn insert_memory(connection: &mut Connection, memory: &MemoryDraft) -> Result<i64, String> {
''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''               kind, content, importance, source_event_id, created_at_ms, updated_at_ms\n\
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
''',
    '''               kind, content, importance, source_event_id, origin, created_at_ms, updated_at_ms\n\
             ) VALUES (?1, ?2, ?3, ?4, 'automatic', ?5, ?5)",
''',
)

# Explicit v2 -> v3 migration regression: all pre-evaluator rows become manual safely.
literal(
    "src-tauri/src/persistence.rs",
    '''    #[test]
    fn migration_from_v1_keeps_pet_state_and_adds_v2_tables() {
''',
    '''    #[test]
    fn migration_from_v2_adds_durable_manual_origin() {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        connection
            .execute_batch(
                "CREATE TABLE pet_state (\n\
                   singleton INTEGER PRIMARY KEY CHECK (singleton = 1),\n\
                   facing TEXT NOT NULL, energy REAL NOT NULL, curiosity REAL NOT NULL,\n\
                   bond REAL NOT NULL, sleep_pressure REAL NOT NULL, updated_at_ms INTEGER NOT NULL\n\
                 );\n\
                 CREATE TABLE activity_journal (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT, event_type TEXT NOT NULL,\n\
                   category TEXT NOT NULL, created_at_ms INTEGER NOT NULL\n\
                 );\n\
                 CREATE TABLE memories (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT, kind TEXT NOT NULL, content TEXT NOT NULL,\n\
                   importance REAL NOT NULL, source_event_id INTEGER, created_at_ms INTEGER NOT NULL,\n\
                   updated_at_ms INTEGER NOT NULL, last_accessed_at_ms INTEGER\n\
                 );\n\
                 INSERT INTO memories(kind, content, importance, source_event_id, created_at_ms, updated_at_ms)\n\
                   VALUES ('preference', 'pre evaluator record', 0.7, NULL, 1, 1);\n\
                 PRAGMA user_version = 2;",
            )
            .expect("seed v2 schema");

        let bootstrap = PersistenceBootstrap::from_connection(connection).expect("migrate v2");
        let origin: String = bootstrap
            .connection
            .query_row("SELECT origin FROM memories LIMIT 1", [], |row| row.get(0))
            .expect("origin column");
        assert_eq!(origin, "manual");
        let version: i64 = bootstrap
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .expect("schema version");
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn migration_from_v1_keeps_pet_state_and_adds_v2_tables() {
''',
)

# Frontend exposes provenance explicitly instead of inferring manual/automatic from nullable FK.
literal(
    "src/lib/memory/runtime.ts",
    "export type MemoryKind = 'episodic' | 'semantic' | 'preference' | 'relationship';",
    "export type MemoryKind = 'episodic' | 'semantic' | 'preference' | 'relationship';\nexport type MemoryOrigin = 'manual' | 'automatic';",
)
literal(
    "src/lib/memory/runtime.ts",
    '''  sourceEventId: number | null;
}
''',
    '''  sourceEventId: number | null;
  origin: MemoryOrigin;
}
''',
)
literal(
    "src/lib/ui/companion/MemorySection.svelte",
    '''  function memorySourceLabel(memory: MemoryRecord) {
    if (memory.sourceEventId === null) return '來源 · 手動建立';
    const source = memorySources.get(memory.sourceEventId);
    if (!source) return `來源事件 #${memory.sourceEventId}`;
    return `來源 · ${activityLabel(source)} · ${memoryTime(source.createdAtMs)}`;
  }
''',
    '''  function memorySourceLabel(memory: MemoryRecord) {
    if (memory.origin === 'manual') return '來源 · 手動建立';
    if (memory.sourceEventId === null) return '來源 · 自動評估（原始事件已依 retention 清除）';
    const source = memorySources.get(memory.sourceEventId);
    if (!source) return `來源 · 自動評估 · 事件 #${memory.sourceEventId}`;
    return `來源 · 自動評估 · ${activityLabel(source)} · ${memoryTime(source.createdAtMs)}`;
  }
''',
)

# ---------------------------------------------------------------------------
# Windows PowerShell 5.1 compatibility for the real target-machine harness.
# ---------------------------------------------------------------------------
literal(
    "scripts/measure-target-baseline.ps1",
    '''function Percentile([double[]]$Values, [double]$P) {
''',
    '''function NumericOrZero($Value) {
  if ($null -eq $Value) { return 0.0 }
  return [double]$Value
}

function Percentile([double[]]$Values, [double]$P) {
''',
)
for field in [
    "cpuAveragePercent",
    "cpuP95Percent",
    "cpuMaxPercent",
    "gpuAverageEngineTotalPercent",
    "gpuP95EngineTotalPercent",
]:
    literal(
        "scripts/measure-target-baseline.ps1",
        f"[double]($summary.{field} ?? 0)",
        f"(NumericOrZero $summary.{field})",
    )
for field in ["workingSetAverageBytes", "workingSetP95Bytes", "privateAverageBytes"]:
    literal(
        "scripts/measure-target-baseline.ps1",
        f"[double]($summary.{field} ?? 0)",
        f"(NumericOrZero $summary.{field})",
    )

# Keep target-script syntax checked by ordinary Windows CI under Windows PowerShell itself.
literal(
    ".github/workflows/windows-ci.yml",
    '''      - name: Reject tracked credential-like source material
        run: npm run validate:secrets

''',
    '''      - name: Reject tracked credential-like source material
        run: npm run validate:secrets

      - name: Parse target benchmark harness with Windows PowerShell 5.1
        shell: powershell
        run: |
          $tokens = $null
          $errors = $null
          [System.Management.Automation.Language.Parser]::ParseFile(
            (Resolve-Path 'scripts/measure-target-baseline.ps1'),
            [ref]$tokens,
            [ref]$errors
          ) | Out-Null
          if ($errors.Count -gt 0) {
            $errors | ForEach-Object { Write-Error $_.Message }
            exit 1
          }

''',
)

literal(
    "docs/LOCAL_DATA_POLICY.md",
    "The application must not create a second competing SQLite owner just to implement management actions.",
    "The application must not create a second competing SQLite owner just to implement management actions. Automatic-memory provenance is persisted independently from journal foreign keys, so retention pruning cannot turn evaluator memories into apparent manual records.",
)

print("Pre-AI semantic hardening applied.")
