use serde::Deserialize;

use crate::domain::events::DomainEvent;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PetInteraction {
    Hover,
    Touch,
    Pet,
    Play,
    FocusStart,
    FocusStop,
    UserReturned,
}

impl PetInteraction {
    pub fn into_event(self) -> DomainEvent {
        match self {
            Self::Hover => DomainEvent::CursorEnteredPet,
            Self::Touch => DomainEvent::PetTouched,
            Self::Pet => DomainEvent::PetPetted,
            Self::Play => DomainEvent::PetPlayRequested,
            Self::FocusStart => DomainEvent::FocusModeStarted,
            Self::FocusStop => DomainEvent::FocusModeEnded,
            Self::UserReturned => DomainEvent::UserReturned,
        }
    }
}
