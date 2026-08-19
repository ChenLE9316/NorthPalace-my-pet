use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Locomotion {
    Stationary,
    Walk,
    // Reserved for production locomotion/animation families not yet emitted by PetBrainV2.
    #[allow(dead_code)]
    Run,
    #[allow(dead_code)]
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
    Held,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Attention {
    Idle,
    User,
    Cursor,
    Window,
    // Reserved for future object-level structured attention; current producers stop at window/cursor.
    #[allow(dead_code)]
    Object,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Emotion {
    Calm,
    Curious,
    Happy,
    // Reserved for deeper personality/relationship reactions.
    #[allow(dead_code)]
    Shy,
    #[allow(dead_code)]
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
    // Reserved for the future unloadable cognition layer; AI is intentionally not implemented yet.
    #[allow(dead_code)]
    Listening,
    #[allow(dead_code)]
    Thinking,
    #[allow(dead_code)]
    Speaking,
    #[allow(dead_code)]
    Remembering,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
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
