use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    Episodic,
    Semantic,
    Preference,
    Relationship,
}

impl MemoryKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Episodic => "episodic",
            Self::Semantic => "semantic",
            Self::Preference => "preference",
            Self::Relationship => "relationship",
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        match value {
            "episodic" => Some(Self::Episodic),
            "semantic" => Some(Self::Semantic),
            "preference" => Some(Self::Preference),
            "relationship" => Some(Self::Relationship),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInput {
    pub kind: MemoryKind,
    pub content: String,
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MemoryRecord {
    pub id: i64,
    pub kind: MemoryKind,
    pub content: String,
    pub importance: f32,
    pub created_at_ms: i64,
    pub updated_at_ms: i64,
    pub source_event_id: Option<i64>,
}

#[derive(Clone, Default)]
pub struct MemoryAdminService {
    database_path: Arc<RwLock<Option<PathBuf>>>,
}

impl MemoryAdminService {
    pub fn install(&self, path: PathBuf) -> Result<(), String> {
        self.database_path
            .write()
            .map(|mut slot| *slot = Some(path))
            .map_err(|_| "memory-admin path lock is poisoned".to_owned())
    }

    pub fn list(
        &self,
        kind: Option<MemoryKind>,
        limit: u32,
    ) -> Result<Vec<MemoryRecord>, String> {
        let connection = self.connection()?;
        list_memories(&connection, kind, limit)
    }

    pub fn search(&self, query: &str, limit: u32) -> Result<Vec<MemoryRecord>, String> {
        let connection = self.connection()?;
        search_memories(&connection, query, limit)
    }

    pub fn create(&self, input: MemoryInput) -> Result<i64, String> {
        validate_input(&input)?;
        let connection = self.connection()?;
        create_memory(&connection, &input)
    }

    pub fn update(&self, id: i64, input: MemoryInput) -> Result<(), String> {
        validate_id(id)?;
        validate_input(&input)?;
        let connection = self.connection()?;
        update_memory(&connection, id, &input)
    }

    pub fn delete(&self, id: i64) -> Result<(), String> {
        validate_id(id)?;
        let connection = self.connection()?;
        delete_memory(&connection, id)
    }

    fn connection(&self) -> Result<Connection, String> {
        let path = self
            .database_path
            .read()
            .map_err(|_| "memory-admin path lock is poisoned".to_owned())?
            .clone()
            .ok_or_else(|| "persistent memory is unavailable for this session".to_owned())?;

        open_admin_connection(&path)
    }
}

fn open_admin_connection(path: &Path) -> Result<Connection, String> {
    let connection = Connection::open(path)
        .map_err(|error| format!("failed to open memory database {}: {error}", path.display()))?;
    connection
        .execute_batch(
            "PRAGMA foreign_keys = ON;\n\
             PRAGMA busy_timeout = 2500;",
        )
        .map_err(|error| format!("failed to configure memory database connection: {error}"))?;
    Ok(connection)
}

fn validate_id(id: i64) -> Result<(), String> {
    if id <= 0 {
        Err("memory id must be positive".to_owned())
    } else {
        Ok(())
    }
}

fn validate_input(input: &MemoryInput) -> Result<(), String> {
    let content = input.content.trim();
    if content.is_empty() {
        return Err("memory content cannot be empty".to_owned());
    }
    if content.chars().count() > 10_000 {
        return Err("memory content is too long (maximum 10,000 characters)".to_owned());
    }
    if !input.importance.is_finite() || !(0.0..=1.0).contains(&input.importance) {
        return Err("memory importance must be between 0 and 1".to_owned());
    }
    Ok(())
}

fn list_memories(
    connection: &Connection,
    kind: Option<MemoryKind>,
    limit: u32,
) -> Result<Vec<MemoryRecord>, String> {
    let limit = limit.clamp(1, 100);
    let sql_all =
        "SELECT id, kind, content, importance, created_at_ms, updated_at_ms, source_event_id\n\
         FROM memories\n\
         ORDER BY updated_at_ms DESC, id DESC\n\
         LIMIT ?1";
    let sql_kind =
        "SELECT id, kind, content, importance, created_at_ms, updated_at_ms, source_event_id\n\
         FROM memories\n\
         WHERE kind = ?1\n\
         ORDER BY updated_at_ms DESC, id DESC\n\
         LIMIT ?2";

    let mut records = Vec::new();
    if let Some(kind) = kind {
        let mut statement = connection
            .prepare(sql_kind)
            .map_err(|error| format!("failed to prepare memory list: {error}"))?;
        let rows = statement
            .query_map(params![kind.as_str(), limit], read_memory_row)
            .map_err(|error| format!("failed to list memories: {error}"))?;
        for row in rows {
            if let Some(record) = row
                .map_err(|error| format!("failed to read memory list row: {error}"))?
            {
                records.push(record);
            }
        }
    } else {
        let mut statement = connection
            .prepare(sql_all)
            .map_err(|error| format!("failed to prepare memory list: {error}"))?;
        let rows = statement
            .query_map(params![limit], read_memory_row)
            .map_err(|error| format!("failed to list memories: {error}"))?;
        for row in rows {
            if let Some(record) = row
                .map_err(|error| format!("failed to read memory list row: {error}"))?
            {
                records.push(record);
            }
        }
    }
    Ok(records)
}

fn search_memories(
    connection: &Connection,
    query: &str,
    limit: u32,
) -> Result<Vec<MemoryRecord>, String> {
    let query = build_fts_query(query);
    if query.is_empty() {
        return list_memories(connection, None, limit);
    }

    let mut statement = connection
        .prepare(
            "SELECT m.id, m.kind, m.content, m.importance, m.created_at_ms,\n\
                    m.updated_at_ms, m.source_event_id\n\
             FROM memory_fts\n\
             JOIN memories m ON m.id = memory_fts.rowid\n\
             WHERE memory_fts MATCH ?1\n\
             ORDER BY bm25(memory_fts), m.importance DESC, m.updated_at_ms DESC\n\
             LIMIT ?2",
        )
        .map_err(|error| format!("failed to prepare memory search: {error}"))?;

    let rows = statement
        .query_map(params![query, limit.clamp(1, 100)], read_memory_row)
        .map_err(|error| format!("failed to search memories: {error}"))?;
    let mut records = Vec::new();
    for row in rows {
        if let Some(record) = row
            .map_err(|error| format!("failed to read memory search row: {error}"))?
        {
            records.push(record);
        }
    }
    Ok(records)
}

fn build_fts_query(query: &str) -> String {
    query
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .map(|part| format!("\"{}\"", part.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" AND ")
}

fn create_memory(connection: &Connection, input: &MemoryInput) -> Result<i64, String> {
    let now = now_ms();
    connection
        .execute(
            "INSERT INTO memories (\n\
               kind, content, importance, source_event_id, created_at_ms, updated_at_ms\n\
             ) VALUES (?1, ?2, ?3, NULL, ?4, ?4)",
            params![
                input.kind.as_str(),
                input.content.trim(),
                input.importance,
                now
            ],
        )
        .map_err(|error| format!("failed to create memory: {error}"))?;
    Ok(connection.last_insert_rowid())
}

fn update_memory(connection: &Connection, id: i64, input: &MemoryInput) -> Result<(), String> {
    let changed = connection
        .execute(
            "UPDATE memories\n\
             SET kind = ?1, content = ?2, importance = ?3, updated_at_ms = ?4\n\
             WHERE id = ?5",
            params![
                input.kind.as_str(),
                input.content.trim(),
                input.importance,
                now_ms(),
                id
            ],
        )
        .map_err(|error| format!("failed to update memory: {error}"))?;

    if changed == 0 {
        Err(format!("memory {id} was not found"))
    } else {
        Ok(())
    }
}

fn delete_memory(connection: &Connection, id: i64) -> Result<(), String> {
    let changed = connection
        .execute("DELETE FROM memories WHERE id = ?1", params![id])
        .map_err(|error| format!("failed to delete memory: {error}"))?;
    if changed == 0 {
        Err(format!("memory {id} was not found"))
    } else {
        Ok(())
    }
}

fn read_memory_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Option<MemoryRecord>> {
    let kind_raw: String = row.get(1)?;
    let Some(kind) = MemoryKind::from_str(&kind_raw) else {
        return Ok(None);
    };

    Ok(Some(MemoryRecord {
        id: row.get(0)?,
        kind,
        content: row.get(2)?,
        importance: row.get(3)?,
        created_at_ms: row.get(4)?,
        updated_at_ms: row.get(5)?,
        source_event_id: row.get(6)?,
    }))
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

    fn test_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("in-memory SQLite");
        connection
            .execute_batch(
                "PRAGMA foreign_keys = ON;\n\
                 CREATE TABLE activity_journal (\n\
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
                   last_accessed_at_ms INTEGER,\n\
                   FOREIGN KEY(source_event_id) REFERENCES activity_journal(id) ON DELETE SET NULL\n\
                 );\n\
                 CREATE VIRTUAL TABLE memory_fts USING fts5(\n\
                   content, content='memories', content_rowid='id', tokenize='unicode61'\n\
                 );\n\
                 CREATE TRIGGER memories_ai AFTER INSERT ON memories BEGIN\n\
                   INSERT INTO memory_fts(rowid, content) VALUES (new.id, new.content);\n\
                 END;\n\
                 CREATE TRIGGER memories_ad AFTER DELETE ON memories BEGIN\n\
                   INSERT INTO memory_fts(memory_fts, rowid, content) VALUES ('delete', old.id, old.content);\n\
                 END;\n\
                 CREATE TRIGGER memories_au AFTER UPDATE ON memories BEGIN\n\
                   INSERT INTO memory_fts(memory_fts, rowid, content) VALUES ('delete', old.id, old.content);\n\
                   INSERT INTO memory_fts(rowid, content) VALUES (new.id, new.content);\n\
                 END;",
            )
            .expect("memory schema");
        connection
    }

    #[test]
    fn create_update_search_and_delete_memory() {
        let connection = test_connection();
        let input = MemoryInput {
            kind: MemoryKind::Preference,
            content: "Quiet companion behavior during focus sessions".to_owned(),
            importance: 0.8,
        };
        let id = create_memory(&connection, &input).expect("create memory");
        let hits = search_memories(&connection, "quiet focus", 10).expect("search");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, id);

        let updated = MemoryInput {
            kind: MemoryKind::Preference,
            content: "Very quiet companion behavior during deep focus".to_owned(),
            importance: 0.9,
        };
        update_memory(&connection, id, &updated).expect("update memory");
        let listed = list_memories(&connection, Some(MemoryKind::Preference), 10)
            .expect("list memories");
        assert_eq!(listed[0].importance, 0.9);
        assert!(listed[0].content.contains("Very quiet"));

        delete_memory(&connection, id).expect("delete memory");
        assert!(list_memories(&connection, None, 10)
            .expect("list after delete")
            .is_empty());
    }

    #[test]
    fn input_validation_rejects_blank_or_invalid_importance() {
        let blank = MemoryInput {
            kind: MemoryKind::Episodic,
            content: "   ".to_owned(),
            importance: 0.5,
        };
        assert!(validate_input(&blank).is_err());

        let invalid = MemoryInput {
            kind: MemoryKind::Episodic,
            content: "valid text".to_owned(),
            importance: 1.5,
        };
        assert!(validate_input(&invalid).is_err());
    }
}
