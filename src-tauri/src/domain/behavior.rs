use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorKind {
    AmbientIdle,
    ObserveUser,
    ReceivePet,
    Play,
    SettleToRest,
    Sleep,
    Wake,
    FocusGuard,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BehaviorIntent {
    pub kind: BehaviorKind,
    pub priority: u8,
    pub remaining_ms: u64,
    pub interruptible: bool,
    pub animation: &'static str,
}

impl BehaviorIntent {
    pub fn receive_pet() -> Self {
        Self {
            kind: BehaviorKind::ReceivePet,
            priority: 60,
            remaining_ms: 3_200,
            interruptible: true,
            animation: "pet_receive",
        }
    }

    pub fn play() -> Self {
        Self {
            kind: BehaviorKind::Play,
            priority: 50,
            remaining_ms: 5_000,
            interruptible: true,
            animation: "play",
        }
    }

    pub fn tick(&mut self, delta_ms: u64) {
        self.remaining_ms = self.remaining_ms.saturating_sub(delta_ms);
    }

    pub fn finished(&self) -> bool {
        self.remaining_ms == 0
    }
}
