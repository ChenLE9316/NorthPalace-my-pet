use crate::domain::{
    behavior::{BehaviorIntent, BehaviorKind},
    events::DomainEvent,
    pet_state::{Attention, Emotion, Locomotion, PetMode, PetStateV2, Posture},
};

#[derive(Debug)]
pub struct PetBrainV2 {
    state: PetStateV2,
    behavior: Option<BehaviorIntent>,
    time_hour: u8,
}

impl Default for PetBrainV2 {
    fn default() -> Self {
        Self {
            state: PetStateV2::default(),
            behavior: None,
            time_hour: 12,
        }
    }
}

impl PetBrainV2 {
    pub fn state(&self) -> PetStateV2 {
        self.state.clone()
    }

    pub fn behavior(&self) -> Option<BehaviorIntent> {
        self.behavior.clone()
    }

    pub fn handle_event(&mut self, event: DomainEvent) {
        match event {
            DomainEvent::Tick { delta_ms } => self.on_tick(delta_ms),
            DomainEvent::UserIdleChanged { idle_ms } => {
                self.state.user_idle_ms = idle_ms;
                self.apply_ambient_policy();
            }
            DomainEvent::UserReturned => {
                self.state.user_idle_ms = 0;
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Happy;
                self.state.locomotion = Locomotion::Stationary;

                if self.state.posture == Posture::Sleep || self.state.posture == Posture::Lie {
                    self.state.posture = Posture::Stand;
                    self.start_behavior(BehaviorIntent::wake());
                } else {
                    self.start_behavior(BehaviorIntent::observe_user());
                }
            }
            DomainEvent::CursorEnteredPet => {
                self.state.attention = Attention::Cursor;
                self.state.emotion = Emotion::Curious;
                self.state.curiosity = (self.state.curiosity + 0.005).clamp(0.0, 1.0);
            }
            DomainEvent::CursorLeftPet => {
                if self.state.attention == Attention::Cursor {
                    self.state.attention = Attention::Idle;
                }
            }
            DomainEvent::PetTouched => {
                self.wake_for_interaction();
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Curious;
            }
            DomainEvent::PetPetted => {
                self.wake_for_interaction();
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Happy;
                self.state.posture = Posture::Sit;
                self.state.bond = (self.state.bond + 0.01).clamp(0.0, 1.0);
                self.start_behavior(BehaviorIntent::receive_pet());
            }
            DomainEvent::PetPlayRequested => {
                self.wake_for_interaction();
                self.state.mode = PetMode::Play;
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Happy;
                self.state.posture = Posture::Stand;
                self.state.locomotion = Locomotion::Walk;
                self.state.energy = (self.state.energy - 0.02).clamp(0.0, 1.0);
                self.state.bond = (self.state.bond + 0.005).clamp(0.0, 1.0);
                self.start_behavior(BehaviorIntent::play());
            }
            DomainEvent::FocusModeStarted => {
                self.state.mode = PetMode::FocusGuard;
                self.state.attention = Attention::Window;
                self.state.emotion = Emotion::Focused;
                self.state.posture = Posture::Sit;
                self.state.locomotion = Locomotion::Stationary;
                self.start_behavior(BehaviorIntent::focus_guard());
            }
            DomainEvent::FocusModeEnded => {
                self.state.mode = PetMode::Ambient;
                self.state.emotion = Emotion::Calm;
                self.state.attention = Attention::User;
                self.state.posture = Posture::Sit;
                if matches!(self.behavior.as_ref().map(|b| b.kind), Some(BehaviorKind::FocusGuard)) {
                    self.behavior = None;
                }
            }
            DomainEvent::ActiveWindowChanged { .. } => {
                if self.state.mode == PetMode::FocusGuard {
                    self.state.attention = Attention::Window;
                }
            }
            DomainEvent::NotificationReceived => {
                if self.state.mode != PetMode::DoNotDisturb && self.state.posture != Posture::Sleep {
                    self.state.attention = Attention::Window;
                    self.state.emotion = Emotion::Curious;
                    self.start_behavior(BehaviorIntent::observe_user());
                }
            }
            DomainEvent::TimeOfDayChanged { hour } => {
                self.time_hour = hour.min(23);
                self.apply_ambient_policy();
            }
            DomainEvent::LlmWorkerStateChanged { available } => {
                self.state.ai_available = available;
            }
        }
    }

    fn on_tick(&mut self, delta_ms: u64) {
        let physiological_delta_ms = delta_ms.min(60_000);
        let hours = physiological_delta_ms as f32 / 3_600_000.0;

        let finished_kind = if let Some(behavior) = self.behavior.as_mut() {
            behavior.tick(delta_ms);
            behavior.finished().then_some(behavior.kind)
        } else {
            None
        };

        if let Some(kind) = finished_kind {
            self.behavior = None;
            self.finish_behavior(kind);
        }

        if self.state.posture == Posture::Sleep {
            self.state.energy = (self.state.energy + 0.18 * hours).clamp(0.0, 1.0);
            self.state.sleep_pressure = (self.state.sleep_pressure - 0.22 * hours).clamp(0.0, 1.0);
        } else {
            self.state.energy = (self.state.energy - 0.025 * hours).clamp(0.0, 1.0);
            self.state.sleep_pressure = (self.state.sleep_pressure + 0.03 * hours).clamp(0.0, 1.0);
        }

        self.apply_ambient_policy();
    }

    fn wake_for_interaction(&mut self) {
        self.state.user_idle_ms = 0;
        if self.state.posture == Posture::Sleep || self.state.posture == Posture::Lie {
            self.state.posture = Posture::Stand;
        }
    }

    fn start_behavior(&mut self, next: BehaviorIntent) {
        if let Some(current) = &self.behavior {
            if !current.interruptible && !current.finished() {
                return;
            }
            if current.priority > next.priority && !current.finished() {
                return;
            }
        }
        self.behavior = Some(next);
    }

    fn finish_behavior(&mut self, kind: BehaviorKind) {
        match kind {
            BehaviorKind::ReceivePet => {
                self.state.emotion = Emotion::Calm;
                self.state.posture = Posture::Sit;
            }
            BehaviorKind::Play => {
                self.state.mode = PetMode::Ambient;
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Stand;
                self.state.emotion = Emotion::Calm;
            }
            BehaviorKind::Wake => {
                self.state.emotion = Emotion::Calm;
                self.state.posture = Posture::Stand;
            }
            BehaviorKind::FocusGuard => {
                if self.state.mode == PetMode::FocusGuard {
                    self.state.emotion = Emotion::Focused;
                    self.state.posture = Posture::Sit;
                }
            }
            BehaviorKind::ObserveUser => {
                if self.state.mode == PetMode::Ambient {
                    self.state.emotion = Emotion::Calm;
                }
            }
            BehaviorKind::SettleToRest => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Lie;
            }
            BehaviorKind::Sleep => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Sleep;
                self.state.emotion = Emotion::Sleepy;
            }
            BehaviorKind::AmbientIdle => {}
        }
    }

    fn apply_ambient_policy(&mut self) {
        if self.behavior.is_some() {
            return;
        }

        match self.state.mode {
            PetMode::FocusGuard => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Sit;
                self.state.emotion = Emotion::Focused;
                return;
            }
            PetMode::DoNotDisturb => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Lie;
                self.state.emotion = Emotion::Calm;
                return;
            }
            PetMode::Play => return,
            PetMode::Ambient => {}
        }

        if self.state.posture == Posture::Sleep {
            return;
        }

        let idle_factor = (self.state.user_idle_ms as f32 / 600_000.0).clamp(0.0, 1.0);
        let low_energy = 1.0 - self.state.energy;
        let night_bonus = if self.time_hour >= 23 || self.time_hour < 6 { 0.15 } else { 0.0 };
        let sleep_score = idle_factor * 0.45
            + self.state.sleep_pressure * 0.35
            + low_energy * 0.20
            + night_bonus;

        if self.state.user_idle_ms >= 120_000 && sleep_score >= 0.72 {
            self.state.locomotion = Locomotion::Stationary;
            self.state.posture = Posture::Sleep;
            self.state.emotion = Emotion::Sleepy;
            self.state.attention = Attention::Idle;
            self.start_behavior(BehaviorIntent::sleep());
        } else if self.state.user_idle_ms >= 180_000 {
            if self.state.posture != Posture::Lie {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Lie;
                self.state.emotion = if self.state.sleep_pressure > 0.5 {
                    Emotion::Sleepy
                } else {
                    Emotion::Calm
                };
                self.start_behavior(BehaviorIntent::settle_to_rest());
            }
        } else if self.state.user_idle_ms >= 60_000 {
            self.state.locomotion = Locomotion::Stationary;
            self.state.posture = Posture::Sit;
            self.state.attention = Attention::Idle;
            self.state.emotion = Emotion::Calm;
        } else {
            self.state.locomotion = Locomotion::Stationary;
            self.state.posture = Posture::Stand;
            if self.state.attention == Attention::Idle {
                self.state.emotion = Emotion::Calm;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pet_reaction_survives_multiple_ticks() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::PetPetted);
        brain.handle_event(DomainEvent::Tick { delta_ms: 1_000 });

        assert_eq!(brain.behavior().map(|b| b.kind), Some(BehaviorKind::ReceivePet));
        assert_eq!(brain.state().emotion, Emotion::Happy);
    }

    #[test]
    fn sleeping_recovers_energy() {
        let mut brain = PetBrainV2::default();
        brain.state.energy = 0.40;
        brain.state.posture = Posture::Sleep;
        let before = brain.state.energy;

        brain.handle_event(DomainEvent::Tick { delta_ms: 60_000 });
        assert!(brain.state().energy > before);
    }

    #[test]
    fn focus_mode_persists_after_entry_behavior() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::FocusModeStarted);
        brain.handle_event(DomainEvent::Tick { delta_ms: 2_000 });

        assert_eq!(brain.state().mode, PetMode::FocusGuard);
        assert_eq!(brain.state().posture, Posture::Sit);
    }
}
