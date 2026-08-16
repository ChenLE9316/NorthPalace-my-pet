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
    pub animation: String,
}

impl BehaviorIntent {
    fn new(
        kind: BehaviorKind,
        priority: u8,
        remaining_ms: u64,
        interruptible: bool,
        animation: &str,
    ) -> Self {
        Self {
            kind,
            priority,
            remaining_ms,
            interruptible,
            animation: animation.to_owned(),
        }
    }

    pub fn observe_user() -> Self {
        Self::new(BehaviorKind::ObserveUser, 30, 1_800, true, "observe_user")
    }

    pub fn receive_pet() -> Self {
        Self::new(BehaviorKind::ReceivePet, 60, 3_200, true, "pet_receive")
    }

    pub fn play() -> Self {
        Self::new(BehaviorKind::Play, 50, 5_000, true, "play")
    }

    pub fn settle_to_rest() -> Self {
        Self::new(BehaviorKind::SettleToRest, 20, 2_000, true, "lie_down")
    }

    pub fn sleep() -> Self {
        Self::new(BehaviorKind::Sleep, 25, 1_600, true, "sleep_enter")
    }

    pub fn wake() -> Self {
        Self::new(BehaviorKind::Wake, 80, 1_500, false, "wake")
    }

    pub fn focus_guard() -> Self {
        Self::new(BehaviorKind::FocusGuard, 70, 1_400, true, "focus_guard_enter")
    }

    pub fn tick(&mut self, delta_ms: u64) {
        self.remaining_ms = self.remaining_ms.saturating_sub(delta_ms);
    }

    pub fn finished(&self) -> bool {
        self.remaining_ms == 0
    }
}
