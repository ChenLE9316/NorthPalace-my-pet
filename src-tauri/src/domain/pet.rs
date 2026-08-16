use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PetActivity {
    Idle,
    Observe,
    Walk,
    Sit,
    Rest,
    Sleep,
    FocusGuard,
    Thinking,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    Calm,
    Curious,
    Happy,
    Sleepy,
    Focused,
}

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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PetState {
    pub activity: PetActivity,
    pub mood: Mood,
    pub energy: f32,
    pub curiosity: f32,
    pub bond: f32,
    pub focus: f32,
    pub idle_seconds: u64,
    pub ai_available: bool,
}

#[derive(Debug)]
pub struct PetBrain {
    state: PetState,
}

impl Default for PetBrain {
    fn default() -> Self {
        Self {
            state: PetState {
                activity: PetActivity::Idle,
                mood: Mood::Calm,
                energy: 0.82,
                curiosity: 0.62,
                bond: 0.20,
                focus: 0.50,
                idle_seconds: 0,
                ai_available: false,
            },
        }
    }
}

impl PetBrain {
    pub fn state(&self) -> PetState {
        self.state.clone()
    }

    pub fn tick(&mut self, seconds: u64) {
        self.state.idle_seconds = self.state.idle_seconds.saturating_add(seconds);
        self.state.energy = (self.state.energy - seconds as f32 * 0.00008).clamp(0.0, 1.0);

        if self.state.activity == PetActivity::FocusGuard {
            self.state.mood = Mood::Focused;
            self.state.focus = (self.state.focus + seconds as f32 * 0.0004).clamp(0.0, 1.0);
            return;
        }

        let idle = self.state.idle_seconds;
        self.state.activity = match idle {
            0..=14 => PetActivity::Idle,
            15..=59 => PetActivity::Observe,
            60..=179 => PetActivity::Sit,
            180..=599 => PetActivity::Rest,
            _ => PetActivity::Sleep,
        };

        self.state.mood = if self.state.activity == PetActivity::Sleep {
            Mood::Sleepy
        } else {
            Mood::Calm
        };
    }

    pub fn interact(&mut self, kind: PetInteraction) {
        match kind {
            PetInteraction::Hover => {
                self.state.activity = PetActivity::Observe;
                self.state.mood = Mood::Curious;
                self.state.curiosity = (self.state.curiosity + 0.01).clamp(0.0, 1.0);
            }
            PetInteraction::Touch => {
                self.state.activity = PetActivity::Observe;
                self.state.mood = Mood::Curious;
            }
            PetInteraction::Pet => {
                self.state.activity = PetActivity::Sit;
                self.state.mood = Mood::Happy;
                self.state.bond = (self.state.bond + 0.01).clamp(0.0, 1.0);
            }
            PetInteraction::Play => {
                self.state.activity = PetActivity::Walk;
                self.state.mood = Mood::Happy;
                self.state.energy = (self.state.energy - 0.02).clamp(0.0, 1.0);
                self.state.bond = (self.state.bond + 0.005).clamp(0.0, 1.0);
            }
            PetInteraction::FocusStart => {
                self.state.activity = PetActivity::FocusGuard;
                self.state.mood = Mood::Focused;
            }
            PetInteraction::FocusStop => {
                self.state.activity = PetActivity::Sit;
                self.state.mood = Mood::Calm;
            }
            PetInteraction::UserReturned => {
                self.state.activity = PetActivity::Observe;
                self.state.mood = Mood::Happy;
            }
        }

        self.state.idle_seconds = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pet_sleeps_after_long_idle() {
        let mut brain = PetBrain::default();
        brain.tick(600);
        assert_eq!(brain.state().activity, PetActivity::Sleep);
    }

    #[test]
    fn petting_increases_bond() {
        let mut brain = PetBrain::default();
        let before = brain.state().bond;
        brain.interact(PetInteraction::Pet);
        assert!(brain.state().bond > before);
        assert_eq!(brain.state().mood, Mood::Happy);
    }
}
