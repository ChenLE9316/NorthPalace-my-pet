use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Locomotion {
    Stationary,
    Walk,
    Run,
    Jump,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Facing {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    Stand,
    Sit,
    Lie,
    Sleep,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Attention {
    Idle,
    User,
    Cursor,
    Window,
    Object,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Emotion {
    Calm,
    Curious,
    Happy,
    Shy,
    Concerned,
    Sleepy,
    Focused,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PetMode {
    Ambient,
    FocusGuard,
    DoNotDisturb,
    Play,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CognitionState {
    Idle,
    Listening,
    Thinking,
    Speaking,
    Remembering,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PetStateV2 {
    pub locomotion: Locomotion,
    pub facing: Facing,
    pub posture: Posture,
    pub attention: Attention,
    pub emotion: Emotion,
    pub mode: PetMode,
    pub cognition: CognitionState,
    pub energy: f32,
    pub curiosity: f32,
    pub bond: f32,
    pub sleep_pressure: f32,
    pub user_idle_ms: u64,
    pub ai_available: bool,
}

impl Default for PetStateV2 {
    fn default() -> Self {
        Self {
            locomotion: Locomotion::Stationary,
            facing: Facing::Right,
            posture: Posture::Stand,
            attention: Attention::Idle,
            emotion: Emotion::Calm,
            mode: PetMode::Ambient,
            cognition: CognitionState::Idle,
            energy: 0.82,
            curiosity: 0.62,
            bond: 0.20,
            sleep_pressure: 0.10,
            user_idle_ms: 0,
            ai_available: false,
        }
    }
}
