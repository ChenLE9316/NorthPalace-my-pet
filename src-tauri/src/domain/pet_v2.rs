use crate::domain::{
    behavior::{BehaviorIntent, BehaviorKind},
    events::DomainEvent,
    personality::{AmbientAction, AmbientContext, PersonalityProfile},
    pet_state::{Attention, Emotion, Locomotion, PetMode, PetStateV2, Posture},
};

#[derive(Debug)]
pub struct PetBrainV2 {
    state: PetStateV2,
    behavior: Option<BehaviorIntent>,
    personality: PersonalityProfile,
    time_hour: u8,
    ambient_elapsed_ms: u64,
    ambient_decision_index: u64,
}

impl Default for PetBrainV2 {
    fn default() -> Self {
        Self::from_state(PetStateV2::default())
    }
}

impl PetBrainV2 {
    pub fn from_state(state: PetStateV2) -> Self {
        Self {
            state,
            behavior: None,
            personality: PersonalityProfile::canonical_lenvu(),
            time_hour: 12,
            ambient_elapsed_ms: 0,
            ambient_decision_index: 0,
        }
    }

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
                self.ambient_elapsed_ms = 0;
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Happy;
                self.state.locomotion = Locomotion::Stationary;

                if self.state.posture == Posture::Held {
                    return;
                }

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
                if self.state.posture != Posture::Held {
                    self.state.posture = Posture::Sit;
                }
                self.state.bond = (self.state.bond + 0.01).clamp(0.0, 1.0);
                self.start_behavior(BehaviorIntent::receive_pet());
            }
            DomainEvent::PetPlayRequested => {
                if self.state.posture == Posture::Held {
                    return;
                }
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
            DomainEvent::PetPickedUp => {
                self.state.user_idle_ms = 0;
                self.ambient_elapsed_ms = 0;
                self.behavior = None;
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Held;
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Curious;
            }
            DomainEvent::PetDropped => {
                if self.state.posture != Posture::Held {
                    return;
                }
                self.ambient_elapsed_ms = 0;
                self.state.locomotion = Locomotion::Stationary;
                if self.state.mode == PetMode::FocusGuard {
                    self.state.posture = Posture::Sit;
                    self.state.attention = Attention::Window;
                    self.state.emotion = Emotion::Focused;
                    self.start_behavior(BehaviorIntent::focus_guard());
                } else {
                    self.state.posture = Posture::Stand;
                    self.state.attention = Attention::User;
                    self.state.emotion = Emotion::Calm;
                    self.start_behavior(BehaviorIntent::observe_user());
                }
            }
            DomainEvent::FocusModeStarted => {
                self.ambient_elapsed_ms = 0;
                self.state.mode = PetMode::FocusGuard;
                self.state.attention = Attention::Window;
                self.state.emotion = Emotion::Focused;
                self.state.locomotion = Locomotion::Stationary;
                if self.state.posture != Posture::Held {
                    self.state.posture = Posture::Sit;
                    self.start_behavior(BehaviorIntent::focus_guard());
                }
            }
            DomainEvent::FocusModeEnded => {
                self.ambient_elapsed_ms = 0;
                self.state.mode = PetMode::Ambient;
                self.state.emotion = Emotion::Calm;
                self.state.attention = Attention::User;
                if self.state.posture != Posture::Held {
                    self.state.posture = Posture::Sit;
                }
                if matches!(
                    self.behavior.as_ref().map(|b| b.kind),
                    Some(BehaviorKind::FocusGuard)
                ) {
                    self.behavior = None;
                }
            }
            DomainEvent::ActiveWindowChanged { .. } => {
                if self.state.mode == PetMode::FocusGuard {
                    self.state.attention = Attention::Window;
                }
            }
            DomainEvent::NotificationReceived => {
                if self.state.mode != PetMode::DoNotDisturb
                    && self.state.posture != Posture::Sleep
                    && self.state.posture != Posture::Held
                {
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
            DomainEvent::PetFacingChanged { facing } => {
                self.state.facing = facing;
            }
        }
    }

    fn on_tick(&mut self, delta_ms: u64) {
        let physiological_delta_ms = delta_ms.min(60_000);
        let decision_delta_ms = delta_ms.min(5_000);
        let hours = physiological_delta_ms as f32 / 3_600_000.0;

        if self.state.mode == PetMode::Ambient
            && self.state.posture != Posture::Sleep
            && self.state.posture != Posture::Held
        {
            self.ambient_elapsed_ms = self.ambient_elapsed_ms.saturating_add(decision_delta_ms);
        }

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
        self.ambient_elapsed_ms = 0;
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
            BehaviorKind::Explore => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Stand;
                self.state.emotion = Emotion::Calm;
                self.state.attention = Attention::Idle;
            }
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
                    self.state.attention = Attention::Idle;
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
            BehaviorKind::AmbientIdle => {
                if self.state.mode == PetMode::Ambient && self.state.user_idle_ms < 60_000 {
                    self.state.locomotion = Locomotion::Stationary;
                    self.state.posture = Posture::Stand;
                    self.state.attention = Attention::Idle;
                    self.state.emotion = Emotion::Calm;
                }
            }
        }
    }

    fn apply_ambient_policy(&mut self) {
        if self.behavior.is_some() || self.state.posture == Posture::Held {
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
        let night_bonus = if self.time_hour >= 23 || self.time_hour < 6 {
            0.15
        } else {
            0.0
        };
        let sleep_score =
            idle_factor * 0.45 + self.state.sleep_pressure * 0.35 + low_energy * 0.20 + night_bonus;

        if self.state.user_idle_ms >= 120_000 && sleep_score >= 0.72 {
            self.state.locomotion = Locomotion::Stationary;
            self.state.posture = Posture::Sleep;
            self.state.emotion = Emotion::Sleepy;
            self.state.attention = Attention::Idle;
            self.ambient_elapsed_ms = 0;
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
                self.ambient_elapsed_ms = 0;
                self.start_behavior(BehaviorIntent::settle_to_rest());
            }
        } else if self.state.user_idle_ms >= 60_000 {
            self.state.locomotion = Locomotion::Stationary;
            self.state.posture = Posture::Sit;
            self.state.attention = Attention::Idle;
            self.state.emotion = Emotion::Calm;
        } else if let Some(action) = self.personality.choose_ambient_action(
            AmbientContext {
                elapsed_ms: self.ambient_elapsed_ms,
                user_idle_ms: self.state.user_idle_ms,
                energy: self.state.energy,
                curiosity: self.state.curiosity,
                bond: self.state.bond,
                sleep_pressure: self.state.sleep_pressure,
                hour: self.time_hour,
            },
            self.ambient_decision_index,
        ) {
            self.ambient_elapsed_ms = 0;
            self.ambient_decision_index = self.ambient_decision_index.wrapping_add(1);
            self.apply_ambient_action(action);
        } else {
            self.state.locomotion = Locomotion::Stationary;
            self.state.posture = Posture::Stand;
            if self.state.attention == Attention::Idle {
                self.state.emotion = Emotion::Calm;
            }
        }
    }

    fn apply_ambient_action(&mut self, action: AmbientAction) {
        match action {
            AmbientAction::Explore => {
                self.state.locomotion = Locomotion::Walk;
                self.state.posture = Posture::Stand;
                self.state.attention = Attention::Idle;
                self.state.emotion = Emotion::Curious;
                self.start_behavior(BehaviorIntent::explore());
            }
            AmbientAction::Observe => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Stand;
                self.state.attention = Attention::User;
                self.state.emotion = Emotion::Curious;
                self.start_behavior(BehaviorIntent::observe_user());
            }
            AmbientAction::Sit => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Sit;
                self.state.attention = Attention::Idle;
                self.state.emotion = Emotion::Calm;
                self.start_behavior(BehaviorIntent::ambient_sit());
            }
            AmbientAction::Stay => {
                self.state.locomotion = Locomotion::Stationary;
                self.state.posture = Posture::Stand;
                self.state.attention = Attention::Idle;
                self.state.emotion = Emotion::Calm;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::pet_state::Facing;

    #[test]
    fn constructor_accepts_persistent_initial_state() {
        let state = PetStateV2 {
            bond: 0.77,
            facing: Facing::Left,
            ..PetStateV2::default()
        };
        let brain = PetBrainV2::from_state(state);
        assert_eq!(brain.state().bond, 0.77);
        assert_eq!(brain.state().facing, Facing::Left);
        assert!(brain.behavior().is_none());
    }

    #[test]
    fn pet_reaction_survives_multiple_ticks() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::PetPetted);
        brain.handle_event(DomainEvent::Tick { delta_ms: 1_000 });

        assert_eq!(
            brain.behavior().map(|b| b.kind),
            Some(BehaviorKind::ReceivePet)
        );
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

    #[test]
    fn facing_is_domain_state() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::PetFacingChanged {
            facing: Facing::Left,
        });
        assert_eq!(brain.state().facing, Facing::Left);
    }

    #[test]
    fn personality_selector_eventually_explores_during_active_ambient_time() {
        let mut brain = PetBrainV2::default();
        let mut saw_explore = false;

        for _ in 0..80 {
            brain.handle_event(DomainEvent::Tick { delta_ms: 5_000 });
            if brain.behavior().map(|b| b.kind) == Some(BehaviorKind::Explore) {
                saw_explore = true;
                break;
            }
        }

        assert!(saw_explore);
    }

    #[test]
    fn focus_guard_blocks_personality_ambient_actions() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::FocusModeStarted);
        for _ in 0..20 {
            brain.handle_event(DomainEvent::Tick { delta_ms: 5_000 });
        }

        assert_eq!(brain.state().mode, PetMode::FocusGuard);
        assert_ne!(
            brain.behavior().map(|b| b.kind),
            Some(BehaviorKind::Explore)
        );
    }

    #[test]
    fn picked_up_posture_survives_runtime_ticks() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::PetPickedUp);
        brain.handle_event(DomainEvent::Tick { delta_ms: 60_000 });

        assert_eq!(brain.state().posture, Posture::Held);
        assert_eq!(brain.state().locomotion, Locomotion::Stationary);
        assert_eq!(brain.state().attention, Attention::User);
    }

    #[test]
    fn dropping_returns_to_stable_ambient_posture() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::PetPickedUp);
        brain.handle_event(DomainEvent::PetDropped);

        assert_eq!(brain.state().posture, Posture::Stand);
        assert_eq!(brain.state().locomotion, Locomotion::Stationary);
        assert_eq!(brain.state().mode, PetMode::Ambient);
    }

    #[test]
    fn focus_guard_is_restored_after_drop() {
        let mut brain = PetBrainV2::default();
        brain.handle_event(DomainEvent::PetPickedUp);
        brain.handle_event(DomainEvent::FocusModeStarted);
        assert_eq!(brain.state().posture, Posture::Held);

        brain.handle_event(DomainEvent::PetDropped);
        assert_eq!(brain.state().mode, PetMode::FocusGuard);
        assert_eq!(brain.state().posture, Posture::Sit);
        assert_eq!(brain.state().emotion, Emotion::Focused);
    }
}
