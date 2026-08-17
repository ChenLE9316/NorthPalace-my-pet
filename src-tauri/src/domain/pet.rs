use serde::Deserialize;

use crate::domain::events::DomainEvent;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PetInteraction {
    Hover,
    HoverEnd,
    Touch,
    Pet,
    Play,
    PickUp,
    Drop,
    FocusStart,
    FocusStop,
    UserReturned,
}

impl PetInteraction {
    pub fn into_event(self) -> DomainEvent {
        match self {
            Self::Hover => DomainEvent::CursorEnteredPet,
            Self::HoverEnd => DomainEvent::CursorLeftPet,
            Self::Touch => DomainEvent::PetTouched,
            Self::Pet => DomainEvent::PetPetted,
            Self::Play => DomainEvent::PetPlayRequested,
            Self::PickUp => DomainEvent::PetPickedUp,
            Self::Drop => DomainEvent::PetDropped,
            Self::FocusStart => DomainEvent::FocusModeStarted,
            Self::FocusStop => DomainEvent::FocusModeEnded,
            Self::UserReturned => DomainEvent::UserReturned,
        }
    }
}
