use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoryRecord {
    pub id: i64,
    pub event_type: String,
    pub category: String,
    pub created_at_ms: i64,
    pub relationship_kind: Option<String>,
    pub bond_delta: Option<f32>,
}

const SELECT_ACTIVITY: &str = "SELECT a.id, a.event_type, a.category, a.created_at_ms, r.kind, r.bond_delta\n\
     FROM activity_journal a\n\
     LEFT JOIN relationship_events r ON r.journal_id = a.id";

pub(crate) fn list_activity(
    connection: &Connection,
    limit: u32,
) -> Result<Vec<ActivityHistoryRecord>, String> {
    let sql = format!("{SELECT_ACTIVITY}\nORDER BY a.created_at_ms DESC, a.id DESC\nLIMIT ?1");
    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| format!("failed to prepare activity history list: {error}"))?;
    let rows = statement
        .query_map(params![limit.clamp(1, 100)], read_activity_row)
        .map_err(|error| format!("failed to list activity history: {error}"))?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row.map_err(|error| format!("failed to read activity history row: {error}"))?);
    }
    Ok(records)
}

pub(crate) fn get_activity(
    connection: &Connection,
    id: i64,
) -> Result<Option<ActivityHistoryRecord>, String> {
    if id <= 0 {
        return Err("activity id must be positive".to_owned());
    }
    let sql = format!("{SELECT_ACTIVITY}\nWHERE a.id = ?1\nLIMIT 1");
    connection
        .query_row(&sql, params![id], read_activity_row)
        .optional()
        .map_err(|error| format!("failed to read activity history record {id}: {error}"))
}

fn read_activity_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ActivityHistoryRecord> {
    Ok(ActivityHistoryRecord {
        id: row.get(0)?,
        event_type: row.get(1)?,
        category: row.get(2)?,
        created_at_ms: row.get(3)?,
        relationship_kind: row.get(4)?,
        bond_delta: row.get(5)?,
    })
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
                 CREATE TABLE relationship_events (\n\
                   id INTEGER PRIMARY KEY AUTOINCREMENT,\n\
                   journal_id INTEGER,\n\
                   kind TEXT NOT NULL,\n\
                   bond_delta REAL NOT NULL DEFAULT 0.0,\n\
                   created_at_ms INTEGER NOT NULL,\n\
                   FOREIGN KEY(journal_id) REFERENCES activity_journal(id) ON DELETE SET NULL\n\
                 );",
            )
            .expect("activity schema");
        connection
    }

    #[test]
    fn activity_history_includes_relationship_provenance() {
        let connection = test_connection();
        connection
            .execute(
                "INSERT INTO activity_journal (event_type, category, created_at_ms) VALUES (?1, ?2, ?3)",
                params!["pet_petted", "relationship", 1000_i64],
            )
            .expect("activity insert");
        let journal_id = connection.last_insert_rowid();
        connection
            .execute(
                "INSERT INTO relationship_events (journal_id, kind, bond_delta, created_at_ms) VALUES (?1, ?2, ?3, ?4)",
                params![journal_id, "affection", 0.01_f32, 1000_i64],
            )
            .expect("relationship insert");

        let listed = list_activity(&connection, 20).expect("activity list");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].event_type, "pet_petted");
        assert_eq!(listed[0].relationship_kind.as_deref(), Some("affection"));
        assert_eq!(listed[0].bond_delta, Some(0.01));

        let fetched = get_activity(&connection, journal_id)
            .expect("activity get")
            .expect("activity exists");
        assert_eq!(fetched.id, journal_id);
    }

    #[test]
    fn activity_list_is_newest_first() {
        let connection = test_connection();
        connection
            .execute(
                "INSERT INTO activity_journal (event_type, category, created_at_ms) VALUES ('older', 'test', 1000)",
                [],
            )
            .expect("older activity");
        connection
            .execute(
                "INSERT INTO activity_journal (event_type, category, created_at_ms) VALUES ('newer', 'test', 2000)",
                [],
            )
            .expect("newer activity");

        let listed = list_activity(&connection, 20).expect("activity list");
        assert_eq!(listed[0].event_type, "newer");
        assert_eq!(listed[1].event_type, "older");
    }
}
