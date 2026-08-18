use serde::{Deserialize, Serialize};

/// Stable memory categories shared by persistence, admin surfaces and future evaluators.
///
/// Keeping this type in the domain prevents SQLite/application adapters from inventing
/// independent category enums that can drift as the memory system evolves.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    Episodic,
    Semantic,
    Preference,
    Relationship,
}

impl MemoryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Episodic => "episodic",
            Self::Semantic => "semantic",
            Self::Preference => "preference",
            Self::Relationship => "relationship",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "episodic" => Some(Self::Episodic),
            "semantic" => Some(Self::Semantic),
            "preference" => Some(Self::Preference),
            "relationship" => Some(Self::Relationship),
            _ => None,
        }
    }
}

/// A normalized long-term memory candidate that can be queued to persistence.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryDraft {
    pub kind: MemoryKind,
    pub content: String,
    pub importance: f32,
    pub source_event_id: Option<i64>,
}

/// Lightweight retrieval result used by application/runtime consumers.
#[derive(Debug, Clone, PartialEq)]
pub struct MemorySearchHit {
    pub id: i64,
    pub kind: MemoryKind,
    pub content: String,
    pub importance: f32,
    pub created_at_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_kind_storage_round_trip_is_stable() {
        for (kind, raw) in [
            (MemoryKind::Episodic, "episodic"),
            (MemoryKind::Semantic, "semantic"),
            (MemoryKind::Preference, "preference"),
            (MemoryKind::Relationship, "relationship"),
        ] {
            assert_eq!(kind.as_str(), raw);
            assert_eq!(MemoryKind::from_str(raw), Some(kind));
        }
        assert_eq!(MemoryKind::from_str("unknown"), None);
    }
}
