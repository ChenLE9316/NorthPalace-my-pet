from pathlib import Path


def load(path: str) -> str:
    return Path(path).read_text(encoding="utf-8")


def save(path: str, text: str) -> None:
    Path(path).write_text(text, encoding="utf-8", newline="\n")


def literal(path: str, old: str, new: str, expected: int = 1) -> None:
    text = load(path)
    count = text.count(old)
    if count != expected:
        raise RuntimeError(
            f"{path}: expected {expected} literal matches, found {count}: {old!r}"
        )
    save(path, text.replace(old, new))


# Equivalent style rewrites requested by Clippy.
literal(
    "src-tauri/src/bootstrap.rs",
    '''    if let Some(data_dir) = &local_data_dir {
        if let Err(error) = privacy_policy_service.install(data_dir.join("privacy-rules.json")) {
            eprintln!(
                "Lenvu privacy rules unavailable; active-window identity remains blocked: {error}"
            );
        }
    }
''',
    '''    if let Some(data_dir) = &local_data_dir
        && let Err(error) = privacy_policy_service.install(data_dir.join("privacy-rules.json"))
    {
        eprintln!(
            "Lenvu privacy rules unavailable; active-window identity remains blocked: {error}"
        );
    }
''',
)
literal(
    "src-tauri/src/bootstrap.rs",
    '''                if !persistence.had_saved_state() {
                    if let Err(error) = persistence.queue_save(initial_state) {
                        eprintln!("Lenvu initial persistence save failed: {error}");
                    }
                }
''',
    '''                if !persistence.had_saved_state()
                    && let Err(error) = persistence.queue_save(initial_state)
                {
                    eprintln!("Lenvu initial persistence save failed: {error}");
                }
''',
)

literal(
    "src-tauri/src/commands.rs",
    '''        return Ok(StartupStatus {
            supported: true,
            enabled,
        });
''',
    '''        Ok(StartupStatus {
            supported: true,
            enabled,
        })
''',
)
literal(
    "src-tauri/src/commands.rs",
    '''        return Ok(StartupStatus {
            supported: true,
            enabled: actual,
        });
''',
    '''        Ok(StartupStatus {
            supported: true,
            enabled: actual,
        })
''',
)

# Domain contract is intentionally ahead of a future object-attention producer.
literal(
    "src-tauri/src/domain/pet_state.rs",
    '''    Window,
    Object,
''',
    '''    Window,
    // Reserved for future object-level structured attention; current producers stop at window/cursor.
    #[allow(dead_code)]
    Object,
''',
)

# Memory Evaluator transport is intentionally staged before its future producer exists.
literal(
    "src-tauri/src/persistence.rs",
    '''    StoreMemory(MemoryDraft),
    SearchMemories {
''',
    '''    #[allow(dead_code)]
    StoreMemory(MemoryDraft),
    #[allow(dead_code)]
    SearchMemories {
''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
''',
    '''    #[allow(dead_code)]
    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
''',
    expected=2,
)
literal(
    "src-tauri/src/persistence.rs",
    '''    pub fn search_memories(
''',
    '''    #[allow(dead_code)]
    pub fn search_memories(
''',
    expected=2,
)

# Convenience helper is currently asserted only by privacy tests.
literal(
    "src-tauri/src/privacy.rs",
    '''    pub fn is_accessibility_context_allowed(&self, app_id: &str) -> bool {
''',
    '''    #[cfg(test)]
    pub fn is_accessibility_context_allowed(&self, app_id: &str) -> bool {
''',
)

# Detached runtime constructors exist only for focused unit tests.
literal(
    "src-tauri/src/runtime.rs",
    '''    thread,
    time::{Duration, Instant},
''',
    '''    time::{Duration, Instant},
''',
)
literal(
    "src-tauri/src/runtime.rs",
    '''};

use serde::Serialize;
''',
    '''};

#[cfg(test)]
use std::thread;

use serde::Serialize;
''',
)
literal(
    "src-tauri/src/runtime.rs",
    '''    pub fn spawn(tick_interval: Duration) -> Self {
''',
    '''    #[cfg(test)]
    pub fn spawn(tick_interval: Duration) -> Self {
''',
)
literal(
    "src-tauri/src/runtime.rs",
    '''    pub fn spawn_with_state(tick_interval: Duration, initial_state: PetStateV2) -> Self {
''',
    '''    #[cfg(test)]
    pub fn spawn_with_state(tick_interval: Duration, initial_state: PetStateV2) -> Self {
''',
)
literal(
    "src-tauri/src/runtime.rs",
    '''    pub fn spawn_with_state_and_observer(
''',
    '''    #[cfg(test)]
    pub fn spawn_with_state_and_observer(
''',
)

# Production uses ordered phase shutdown; whole-supervisor shutdown remains test-only.
literal(
    "src-tauri/src/worker.rs",
    '''    fn cancel_all(&self) {
''',
    '''    #[cfg(test)]
    fn cancel_all(&self) {
''',
)
literal(
    "src-tauri/src/worker.rs",
    '''    pub fn shutdown_and_join(&self, timeout: Duration) -> ShutdownReport {
''',
    '''    #[cfg(test)]
    pub fn shutdown_and_join(&self, timeout: Duration) -> ShutdownReport {
''',
)

# Prefer explicit state construction over mutate-after-default.
literal(
    "src-tauri/src/persistence.rs",
    '''    fn into_runtime_state(self) -> PetStateV2 {
        let mut state = PetStateV2::default();
        state.facing = self.facing;
        state.energy = self.energy.clamp(0.0, 1.0);
        state.curiosity = self.curiosity.clamp(0.0, 1.0);
        state.bond = self.bond.clamp(0.0, 1.0);
        state.sleep_pressure = self.sleep_pressure.clamp(0.0, 1.0);
        state
    }
''',
    '''    fn into_runtime_state(self) -> PetStateV2 {
        PetStateV2 {
            facing: self.facing,
            energy: self.energy.clamp(0.0, 1.0),
            curiosity: self.curiosity.clamp(0.0, 1.0),
            bond: self.bond.clamp(0.0, 1.0),
            sleep_pressure: self.sleep_pressure.clamp(0.0, 1.0),
            ..PetStateV2::default()
        }
    }
''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''            if counts_as_interaction {
                if let Some(hour) = current_hour {
                    persistence.observe_hour(hour, 1)?;
                }
            }
''',
    '''            if counts_as_interaction
                && let Some(hour) = current_hour
            {
                persistence.observe_hour(hour, 1)?;
            }
''',
)
literal(
    "src-tauri/src/persistence.rs",
    '''        let mut state = PetStateV2::default();
        state.bond = 0.91;
''',
    '''        let state = PetStateV2 {
            bond: 0.91,
            ..PetStateV2::default()
        };
''',
)
literal(
    "src-tauri/src/domain/pet_v2.rs",
    '''        let mut state = PetStateV2::default();
        state.bond = 0.77;
        state.facing = Facing::Left;
''',
    '''        let state = PetStateV2 {
            bond: 0.77,
            facing: Facing::Left,
            ..PetStateV2::default()
        };
''',
)
literal(
    "src-tauri/src/runtime.rs",
    '''        let mut state = PetStateV2::default();
        state.bond = 0.66;
        state.facing = Facing::Left;
''',
    '''        let state = PetStateV2 {
            bond: 0.66,
            facing: Facing::Left,
            ..PetStateV2::default()
        };
''',
)

# Preserve the existing ordered shutdown behavior while using Clippy's let-chain form.
literal(
    "src-tauri/src/lib.rs",
    '''        if let Some(runtime) = app_handle.try_state::<RuntimeHandle>() {
            if let Err(error) = runtime.close_event_input() {
                eprintln!("Lenvu runtime input gate could not close cleanly: {error}");
            }
        }
''',
    '''        if let Some(runtime) = app_handle.try_state::<RuntimeHandle>()
            && let Err(error) = runtime.close_event_input()
        {
            eprintln!("Lenvu runtime input gate could not close cleanly: {error}");
        }
''',
)
literal(
    "src-tauri/src/lib.rs",
    '''        if let (Some(snapshot), Some(persistence)) = (
            frozen_snapshot,
            app_handle.try_state::<PersistenceService>(),
        ) {
            if let Err(error) = persistence.save_and_flush(
                snapshot.state,
                bootstrap::PERSISTENCE_FINAL_SAVE_TIMEOUT,
            ) {
                eprintln!("Lenvu final persistence save failed: {error}");
            }
        }
''',
    '''        if let (Some(snapshot), Some(persistence)) = (
            frozen_snapshot,
            app_handle.try_state::<PersistenceService>(),
        ) && let Err(error) = persistence.save_and_flush(
            snapshot.state,
            bootstrap::PERSISTENCE_FINAL_SAVE_TIMEOUT,
        ) {
            eprintln!("Lenvu final persistence save failed: {error}");
        }
''',
)
