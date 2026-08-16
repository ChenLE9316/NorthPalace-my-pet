use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEvent {
    Tick { delta_ms: u64 },
    UserIdleChanged { idle_ms: u64 },
    UserReturned,
    CursorEnteredPet,
    CursorLeftPet,
    PetTouched,
    PetPetted,
    PetPlayRequested,
    FocusModeStarted,
    FocusModeEnded,
    ActiveWindowChanged { app_id: String },
    NotificationReceived,
    TimeOfDayChanged { hour: u8 },
    LlmWorkerStateChanged { available: bool },
}
