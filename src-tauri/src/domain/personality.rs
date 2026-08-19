#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbientAction {
    Stay,
    Observe,
    Sit,
    Explore,
}

#[derive(Debug, Clone, Copy)]
pub struct PersonalityProfile {
    pub curiosity_drive: f32,
    pub calmness: f32,
    pub sociability: f32,
    pub independence: f32,
    pub ambient_decision_interval_ms: u64,
}

impl PersonalityProfile {
    pub const fn canonical_lenvu() -> Self {
        Self {
            curiosity_drive: 0.72,
            calmness: 0.80,
            sociability: 0.58,
            independence: 0.64,
            ambient_decision_interval_ms: 24_000,
        }
    }

    pub fn choose_ambient_action(
        &self,
        context: AmbientContext,
        decision_index: u64,
    ) -> Option<AmbientAction> {
        if context.elapsed_ms < self.ambient_decision_interval_ms || context.user_idle_ms >= 60_000
        {
            return None;
        }

        let weights = self.weights(context);
        let total = weights.explore + weights.observe + weights.sit + weights.stay;
        if total <= f32::EPSILON {
            return Some(AmbientAction::Stay);
        }

        let mut cursor = deterministic_unit(context_seed(context, decision_index)) * total;

        if cursor < weights.explore {
            return Some(AmbientAction::Explore);
        }
        cursor -= weights.explore;

        if cursor < weights.observe {
            return Some(AmbientAction::Observe);
        }
        cursor -= weights.observe;

        if cursor < weights.sit {
            return Some(AmbientAction::Sit);
        }

        Some(AmbientAction::Stay)
    }

    fn weights(&self, context: AmbientContext) -> AmbientWeights {
        let energy = context.energy.clamp(0.0, 1.0);
        let curiosity = context.curiosity.clamp(0.0, 1.0);
        let bond = context.bond.clamp(0.0, 1.0);
        let sleep_pressure = context.sleep_pressure.clamp(0.0, 1.0);
        let active_user = 1.0 - (context.user_idle_ms as f32 / 60_000.0).clamp(0.0, 1.0);
        let wakefulness = energy * (1.0 - sleep_pressure);
        let daytime = if (6..23).contains(&context.hour) {
            1.0
        } else {
            0.18
        };

        let explore = 0.04
            + self.curiosity_drive
                * curiosity
                * 0.95
                * wakefulness
                * daytime
                * (0.55 + self.independence * 0.45);

        let observe = 0.08 + self.sociability * (0.25 + bond * 0.75) * (0.35 + active_user * 0.65);

        let sit = 0.06
            + self.calmness
                * ((1.0 - energy) * 0.45 + sleep_pressure * 0.55 + (1.0 - active_user) * 0.15);

        let stay = 0.22 + self.calmness * 0.25 + (1.0 - wakefulness) * 0.18;

        AmbientWeights {
            explore: explore.max(0.0),
            observe: observe.max(0.0),
            sit: sit.max(0.0),
            stay: stay.max(0.0),
        }
    }
}

impl Default for PersonalityProfile {
    fn default() -> Self {
        Self::canonical_lenvu()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AmbientContext {
    pub elapsed_ms: u64,
    pub user_idle_ms: u64,
    pub energy: f32,
    pub curiosity: f32,
    pub bond: f32,
    pub sleep_pressure: f32,
    pub hour: u8,
}

#[derive(Debug, Clone, Copy)]
struct AmbientWeights {
    explore: f32,
    observe: f32,
    sit: f32,
    stay: f32,
}

fn context_seed(context: AmbientContext, decision_index: u64) -> u64 {
    let energy = quantize(context.energy);
    let curiosity = quantize(context.curiosity);
    let bond = quantize(context.bond);
    let sleep_pressure = quantize(context.sleep_pressure);

    decision_index.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ ((context.hour as u64) << 56)
        ^ (energy << 40)
        ^ (curiosity << 24)
        ^ (bond << 8)
        ^ sleep_pressure
}

fn quantize(value: f32) -> u64 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u64
}

fn deterministic_unit(seed: u64) -> f32 {
    let mut value = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    ((value >> 40) as u32) as f32 / (1_u32 << 24) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_context() -> AmbientContext {
        AmbientContext {
            elapsed_ms: 30_000,
            user_idle_ms: 0,
            energy: 0.82,
            curiosity: 0.62,
            bond: 0.20,
            sleep_pressure: 0.10,
            hour: 12,
        }
    }

    #[test]
    fn decision_waits_for_interval() {
        let profile = PersonalityProfile::canonical_lenvu();
        let mut context = default_context();
        context.elapsed_ms = 10_000;
        assert_eq!(profile.choose_ambient_action(context, 0), None);
    }

    #[test]
    fn decision_is_deterministic_for_same_context_and_index() {
        let profile = PersonalityProfile::canonical_lenvu();
        let context = default_context();
        assert_eq!(
            profile.choose_ambient_action(context, 7),
            profile.choose_ambient_action(context, 7)
        );
    }

    #[test]
    fn curiosity_increases_explore_weight() {
        let profile = PersonalityProfile::canonical_lenvu();
        let mut low = default_context();
        low.curiosity = 0.10;
        let mut high = low;
        high.curiosity = 0.95;
        assert!(profile.weights(high).explore > profile.weights(low).explore);
    }

    #[test]
    fn night_suppresses_explore_weight() {
        let profile = PersonalityProfile::canonical_lenvu();
        let day = default_context();
        let mut night = day;
        night.hour = 2;
        assert!(profile.weights(night).explore < profile.weights(day).explore);
    }

    #[test]
    fn long_user_idle_is_left_to_rest_policy() {
        let profile = PersonalityProfile::canonical_lenvu();
        let mut context = default_context();
        context.user_idle_ms = 60_000;
        assert_eq!(profile.choose_ambient_action(context, 0), None);
    }
}
