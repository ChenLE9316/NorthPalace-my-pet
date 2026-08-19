# Rust CI Diagnostics

Machine-generated on a clean GitHub-hosted windows-latest runner.

- Source commit: 3a0f3feaf0d619d7999a0518f733c4c3b3bdb0dc
- Recorded at (UTC): 2026-08-19T04:28:59.6633424Z
- rustfmt exit code: 1
- Clippy exit code: 101

## rustfmt

``text
[0m[31m-            );
[0m[32m+            let bounds =
[0m[32m+                calculate_bounds(current_monitor.work, window_size.width, window_size.height);
[0m             let current_x = fractional_x.unwrap_or(window_position.x as f64);
             let speed_physical = speed_logical * scale_factor.max(0.5);
             let delta_seconds = delta_seconds.min(0.25);
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\platform\windows\motion.rs:314:
             let projected = projected_x(current_x, active_direction, speed_physical, delta_seconds);
 
             if reaches_edge(projected, active_direction, bounds)
[31m-                && allows_monitor_transition(snapshot.behavior.as_ref().map(|behavior| behavior.kind))
[0m[32m+                && allows_monitor_transition(
[0m[32m+                    snapshot.behavior.as_ref().map(|behavior| behavior.kind),
[0m[32m+                )
[0m             {
                 let adjacent = window.available_monitors().ok().and_then(|monitors| {
                     find_adjacent_monitor(
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\platform\windows\motion.rs:341:
                             target_x,
                             target_bounds.ground_y,
                         ))
[31m-                        .map_err(|error| format!("failed to move pet to adjacent monitor: {error}"))?;
[0m[32m+                        .map_err(|error| {
[0m[32m+                            format!("failed to move pet to adjacent monitor: {error}")
[0m[32m+                        })?;
[0m 
                     if token.wait_timeout(MOTION_TICK) {
                         break;
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\platform\windows\motion.rs:436:
 
     #[test]
     fn initial_direction_respects_domain_facing() {
[31m-        assert_eq!(HorizontalDirection::from_facing(Facing::Left), HorizontalDirection::Left);
[0m[31m-        assert_eq!(HorizontalDirection::from_facing(Facing::Right), HorizontalDirection::Right);
[0m[32m+        assert_eq!(
[0m[32m+            HorizontalDirection::from_facing(Facing::Left),
[0m[32m+            HorizontalDirection::Left
[0m[32m+        );
[0m[32m+        assert_eq!(
[0m[32m+            HorizontalDirection::from_facing(Facing::Right),
[0m[32m+            HorizontalDirection::Right
[0m[32m+        );
[0m     }
 
     #[test]
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:146:
             .write()
             .map_err(|_| "privacy-policy lock is poisoned".to_owned())?;
         if state.fail_closed {
[31m-            return Err("privacy rules are unavailable; sensitive context remains blocked".to_owned());
[0m[32m+            return Err(
[0m[32m+                "privacy rules are unavailable; sensitive context remains blocked".to_owned(),
[0m[32m+            );
[0m         }
         let path = state
             .path
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:248:
 
     #[link(name = "kernel32")]
     unsafe extern "system" {
[31m-        fn MoveFileExW(existing_file_name: *const u16, new_file_name: *const u16, flags: u32) -> i32;
[0m[32m+        fn MoveFileExW(
[0m[32m+            existing_file_name: *const u16,
[0m[32m+            new_file_name: *const u16,
[0m[32m+            flags: u32,
[0m[32m+        ) -> i32;
[0m     }
 
     let source_wide = source
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:300:
             .unwrap_or_default()
             .as_nanos();
         std::env::temp_dir()
[31m-            .join(format!("northpalace-privacy-{}-{nonce}", std::process::id()))
[0m[32m+            .join(format!(
[0m[32m+                "northpalace-privacy-{}-{nonce}",
[0m[32m+                std::process::id()
[0m[32m+            ))
[0m             .join("privacy-rules.json")
     }
 
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:324:
     fn missing_rules_file_installs_with_accessibility_disabled() {
         let path = unique_test_path();
         let privacy = PrivacyPolicyService::default();
[31m-        privacy.install(path.clone()).expect("install privacy rules");
[0m[32m+        privacy
[0m[32m+            .install(path.clone())
[0m[32m+            .expect("install privacy rules");
[0m         let snapshot = privacy.snapshot();
         assert!(!snapshot.fail_closed);
         assert!(!snapshot.accessibility_context_enabled);
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:341:
         if let Some(parent) = path.parent() {
             fs::create_dir_all(parent).expect("create temp privacy directory");
         }
[31m-        fs::write(&path, r#"{"excludedApps":["discord"]}"#)
[0m[31m-            .expect("write legacy privacy rules");
[0m[32m+        fs::write(&path, r#"{"excludedApps":["discord"]}"#).expect("write legacy privacy rules");
[0m 
         let privacy = PrivacyPolicyService::default();
         privacy.install(path.clone()).expect("load legacy rules");
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:360:
     fn exclusions_and_accessibility_capability_persist_and_reload() {
         let path = unique_test_path();
         let privacy = PrivacyPolicyService::default();
[31m-        privacy.install(path.clone()).expect("install privacy rules");
[0m         privacy
[32m+            .install(path.clone())
[0m[32m+            .expect("install privacy rules");
[0m[32m+        privacy
[0m             .add_excluded_app("KeePassXC.EXE")
             .expect("add exclusion");
         let snapshot = privacy
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:373:
         assert!(!privacy.is_accessibility_context_allowed("KEEPASSXC"));
 
         let reloaded = PrivacyPolicyService::default();
[31m-        reloaded.install(path.clone()).expect("reload privacy rules");
[0m[32m+        reloaded
[0m[32m+            .install(path.clone())
[0m[32m+            .expect("reload privacy rules");
[0m         let reloaded_snapshot = reloaded.snapshot();
         assert!(reloaded_snapshot.accessibility_context_enabled);
         assert!(!reloaded.is_accessibility_context_allowed("keepassxc.exe"));
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:393:
     fn replacement_leaves_no_temp_file_after_success() {
         let path = unique_test_path();
         let privacy = PrivacyPolicyService::default();
[31m-        privacy.install(path.clone()).expect("install privacy rules");
[0m[32m+        privacy
[0m[32m+            .install(path.clone())
[0m[32m+            .expect("install privacy rules");
[0m         privacy.add_excluded_app("code").expect("first write");
[31m-        privacy.add_excluded_app("discord").expect("replacement write");
[0m[32m+        privacy
[0m[32m+            .add_excluded_app("discord")
[0m[32m+            .expect("replacement write");
[0m 
         assert!(path.exists());
         assert!(!path.with_extension("json.tmp").exists());
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\privacy.rs:428:
         }
     }
 }
[32m+
[0mDiff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\runtime.rs:1:
 use std::{
[31m-    panic::{catch_unwind, AssertUnwindSafe},
[0m[32m+    panic::{AssertUnwindSafe, catch_unwind},
[0m     sync::{
[31m-        mpsc::{self, Receiver, RecvTimeoutError, Sender},
[0m         Arc, Mutex, RwLock,
[32m+        mpsc::{self, Receiver, RecvTimeoutError, Sender},
[0m     },
     thread,
     time::{Duration, Instant},
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\runtime.rs:12:
 
 use crate::{
     domain::{
[31m-        behavior::BehaviorIntent,
[0m[31m-        events::DomainEvent,
[0m[31m-        pet_state::PetStateV2,
[0m[31m-        pet_v2::PetBrainV2,
[0m[32m+        behavior::BehaviorIntent, events::DomainEvent, pet_state::PetStateV2, pet_v2::PetBrainV2,
[0m     },
     worker::{CancellationToken, WorkerSupervisor},
 };
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\runtime.rs:201:
                     brain.handle_event(event);
                     sequence = sequence.saturating_add(1);
                 }
[31m-                let frozen =
[0m[31m-                    PetRuntimeSnapshot::from_brain(RuntimeHealth::Ready, sequence, &brain);
[0m[32m+                let frozen = PetRuntimeSnapshot::from_brain(RuntimeHealth::Ready, sequence, &brain);
[0m                 publish_snapshot(&snapshot_writer, snapshot_observer.as_ref(), frozen);
                 return;
             }
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\runtime.rs:224:
                 }
                 Err(RecvTimeoutError::Timeout) => {}
                 Err(RecvTimeoutError::Disconnected) => {
[31m-                    let degraded = PetRuntimeSnapshot::from_brain(
[0m[31m-                        RuntimeHealth::Degraded,
[0m[31m-                        sequence,
[0m[31m-                        &brain,
[0m[31m-                    );
[0m[32m+                    let degraded =
[0m[32m+                        PetRuntimeSnapshot::from_brain(RuntimeHealth::Degraded, sequence, &brain);
[0m                     publish_snapshot(&snapshot_writer, snapshot_observer.as_ref(), degraded);
                     return;
                 }
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\runtime.rs:359:
         runtime.close_event_input().expect("close runtime input");
         assert!(runtime.dispatch(DomainEvent::PetPetted).is_err());
 
[31m-        let report = supervisor
[0m[31m-            .shutdown_phase_and_join(WorkerPhase::Runtime, Duration::from_secs(1));
[0m[32m+        let report =
[0m[32m+            supervisor.shutdown_phase_and_join(WorkerPhase::Runtime, Duration::from_secs(1));
[0m         assert_eq!(report.joined, vec!["pet-runtime".to_owned()]);
         assert!(report.detached.is_empty());
         assert_eq!(supervisor.snapshot()[0].health, WorkerHealth::Stopped);
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\shell.rs:1:
 use tauri::{
[32m+    Manager,
[0m     menu::{Menu, MenuItem},
     tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
[31m-    Manager,
[0m };
 
 fn show_companion<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\shell.rs:37:
         true,
         None::<&str>,
     )?;
[31m-    let toggle_pet = MenuItem::with_id(
[0m[31m-        app,
[0m[31m-        "toggle_pet",
[0m[31m-        "Show / Hide Lenvu",
[0m[31m-        true,
[0m[31m-        None::<&str>,
[0m[31m-    )?;
[0m[31m-    let quit = MenuItem::with_id(
[0m[31m-        app,
[0m[31m-        "quit",
[0m[31m-        "Quit NorthPalace-my-pet",
[0m[31m-        true,
[0m[31m-        None::<&str>,
[0m[31m-    )?;
[0m[32m+    let toggle_pet = MenuItem::with_id(app, "toggle_pet", "Show / Hide Lenvu", true, None::<&str>)?;
[0m[32m+    let quit = MenuItem::with_id(app, "quit", "Quit NorthPalace-my-pet", true, None::<&str>)?;
[0m     let tray_menu = Menu::with_items(app, &[&open_companion, &toggle_pet, &quit])?;
 
     let mut tray = TrayIconBuilder::with_id("lenvu")
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\worker.rs:1:
 use std::{
[31m-    panic::{catch_unwind, AssertUnwindSafe},
[0m[32m+    panic::{AssertUnwindSafe, catch_unwind},
[0m     sync::{Arc, Condvar, Mutex},
     thread::{self, JoinHandle},
     time::{Duration, Instant},
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\worker.rs:261:
             .collect()
     }
 
[31m-    pub fn shutdown_phase_and_join(
[0m[31m-        &self,
[0m[31m-        phase: WorkerPhase,
[0m[31m-        timeout: Duration,
[0m[31m-    ) -> ShutdownReport {
[0m[32m+    pub fn shutdown_phase_and_join(&self, phase: WorkerPhase, timeout: Duration) -> ShutdownReport {
[0m         self.inner.cancellation.cancel(phase);
         match self.take_pending(Some(phase)) {
             Ok(pending) => join_pending(pending, timeout),
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\worker.rs:380:
             .expect("spawn worker");
 
         thread::sleep(Duration::from_millis(20));
[31m-        let report = supervisor
[0m[31m-            .shutdown_phase_and_join(WorkerPhase::Producers, Duration::from_secs(1));
[0m[32m+        let report =
[0m[32m+            supervisor.shutdown_phase_and_join(WorkerPhase::Producers, Duration::from_secs(1));
[0m         let (cancelled, elapsed) = rx
             .recv_timeout(Duration::from_secs(1))
             .expect("wait result");
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\worker.rs:418:
             })
             .expect("spawn slow worker");
 
[31m-        let report = supervisor
[0m[31m-            .shutdown_phase_and_join(WorkerPhase::Producers, Duration::from_millis(10));
[0m[32m+        let report =
[0m[32m+            supervisor.shutdown_phase_and_join(WorkerPhase::Producers, Duration::from_millis(10));
[0m         assert_eq!(report.detached, vec!["slow".to_owned()]);
 
         thread::sleep(Duration::from_millis(100));
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\worker.rs:469:
         started.sort_unstable();
         assert_eq!(started, vec!["journal", "producer"]);
 
[31m-        let producer_report = supervisor
[0m[31m-            .shutdown_phase_and_join(WorkerPhase::Producers, Duration::from_secs(1));
[0m[32m+        let producer_report =
[0m[32m+            supervisor.shutdown_phase_and_join(WorkerPhase::Producers, Duration::from_secs(1));
[0m         assert_eq!(producer_report.joined, vec!["producer".to_owned()]);
         assert!(producer_report.detached.is_empty());
 
Diff in \\?\D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri\src\worker.rs:482:
         assert_eq!(journal.phase, WorkerPhase::Journal);
         assert_eq!(journal.health, WorkerHealth::Running);
 
[31m-        let journal_report = supervisor
[0m[31m-            .shutdown_phase_and_join(WorkerPhase::Journal, Duration::from_secs(1));
[0m[32m+        let journal_report =
[0m[32m+            supervisor.shutdown_phase_and_join(WorkerPhase::Journal, Duration::from_secs(1));
[0m         assert_eq!(journal_report.joined, vec!["journal".to_owned()]);
         assert!(journal_report.detached.is_empty());
     }
``

## Clippy

``text
   |
83 | enum PersistenceCommand {
   |      ------------------ variants in this enum
...
94 |     StoreMemory(MemoryDraft),
   |     ^^^^^^^^^^^
95 |     SearchMemories {
   |     ^^^^^^^^^^^^^^

error: methods `queue_memory` and `search_memories` are never used
   --> src\persistence.rs:330:12
    |
300 | impl PersistenceHandle {
    | ---------------------- methods in this implementation
...
330 |     pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
    |            ^^^^^^^^^^^^
...
336 |     pub fn search_memories(
    |            ^^^^^^^^^^^^^^^

error: methods `queue_memory` and `search_memories` are never used
   --> src\persistence.rs:494:12
    |
471 | impl PersistenceService {
    | ----------------------- methods in this implementation
...
494 |     pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {
    |            ^^^^^^^^^^^^
...
501 |     pub fn search_memories(
    |            ^^^^^^^^^^^^^^^

error: method `is_accessibility_context_allowed` is never used
   --> src\privacy.rs:104:12
    |
 52 | impl PrivacyPolicyService {
    | ------------------------- method in this implementation
...
104 |     pub fn is_accessibility_context_allowed(&self, app_id: &str) -> bool {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: associated functions `spawn`, `spawn_with_state`, and `spawn_with_state_and_observer` are never used
  --> src\runtime.rs:62:12
   |
61 | impl RuntimeHandle {
   | ------------------ associated functions in this implementation
62 |     pub fn spawn(tick_interval: Duration) -> Self {
   |            ^^^^^
...
66 |     pub fn spawn_with_state(tick_interval: Duration, initial_state: PetStateV2) -> Self {
   |            ^^^^^^^^^^^^^^^^
...
70 |     pub fn spawn_with_state_and_observer(
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: method `cancel_all` is never used
   --> src\worker.rs:113:8
    |
 99 | impl PhaseCancellation {
    | ---------------------- method in this implementation
...
113 |     fn cancel_all(&self) {
    |        ^^^^^^^^^^

error: method `shutdown_and_join` is never used
   --> src\worker.rs:279:12
    |
163 | impl WorkerSupervisor {
    | --------------------- method in this implementation
...
279 |     pub fn shutdown_and_join(&self, timeout: Duration) -> ShutdownReport {
    |            ^^^^^^^^^^^^^^^^^

error: this `if` statement can be collapsed
  --> src\bootstrap.rs:38:5
   |
38 | /     if let Some(data_dir) = &local_data_dir {
39 | |         if let Err(error) = privacy_policy_service.install(data_dir.join("privacy-rules.json")) {
40 | |             eprintln!(
41 | |                 "Lenvu privacy rules unavailable; active-window identity remains blocked: {error}"
...  |
44 | |     }
   | |_____^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#collapsible_if
   = note: `-D clippy::collapsible-if` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::collapsible_if)]`
help: collapse nested if block
   |
38 ~     if let Some(data_dir) = &local_data_dir
39 ~         && let Err(error) = privacy_policy_service.install(data_dir.join("privacy-rules.json")) {
40 |             eprintln!(
41 |                 "Lenvu privacy rules unavailable; active-window identity remains blocked: {error}"
42 |             );
43 ~         }
   |

error: this `if` statement can be collapsed
  --> src\bootstrap.rs:81:17
   |
81 | /                 if !persistence.had_saved_state() {
82 | |                     if let Err(error) = persistence.queue_save(initial_state) {
83 | |                         eprintln!("Lenvu initial persistence save failed: {error}");
84 | |                     }
85 | |                 }
   | |_________________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#collapsible_if
help: collapse nested if block
   |
81 ~                 if !persistence.had_saved_state()
82 ~                     && let Err(error) = persistence.queue_save(initial_state) {
83 |                         eprintln!("Lenvu initial persistence save failed: {error}");
84 ~                     }
   |

error: unneeded `return` statement
   --> src\commands.rs:190:9
    |
190 | /         return Ok(StartupStatus {
191 | |             supported: true,
192 | |             enabled,
193 | |         });
    | |__________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#needless_return
    = note: `-D clippy::needless-return` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_return)]`
help: remove `return`
    |
190 ~         Ok(StartupStatus {
191 +             supported: true,
192 +             enabled,
193 ~         })
    |

error: unneeded `return` statement
   --> src\commands.rs:229:9
    |
229 | /         return Ok(StartupStatus {
230 | |             supported: true,
231 | |             enabled: actual,
232 | |         });
    | |__________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#needless_return
help: remove `return`
    |
229 ~         Ok(StartupStatus {
230 +             supported: true,
231 +             enabled: actual,
232 ~         })
    |

error: variant `Object` is never constructed
  --> src\domain\pet_state.rs:36:5
   |
31 | pub enum Attention {
   |          --------- variant in this enum
...
36 |     Object,
   |     ^^^^^^
   |
   = note: `Attention` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`

error: field assignment outside of initializer for an instance created with Default::default()
   --> src\persistence.rs:155:9
    |
155 |         state.facing = self.facing;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `domain::pet_state::PetStateV2 { facing: self.facing, energy: self.energy.clamp(0.0, 1.0), curiosity: self.curiosity.clamp(0.0, 1.0), bond: self.bond.clamp(0.0, 1.0), sleep_pressure: self.sleep_pressure.clamp(0.0, 1.0), ..Default::default() }` and removing relevant reassignments
   --> src\persistence.rs:154:9
    |
154 |         let mut state = PetStateV2::default();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#field_reassign_with_default
    = note: `-D clippy::field-reassign-with-default` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::field_reassign_with_default)]`

error: this `if` statement can be collapsed
   --> src\persistence.rs:657:13
    |
657 | /             if counts_as_interaction {
658 | |                 if let Some(hour) = current_hour {
659 | |                     persistence.observe_hour(hour, 1)?;
660 | |                 }
661 | |             }
    | |_____________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#collapsible_if
help: collapse nested if block
    |
657 ~             if counts_as_interaction
658 ~                 && let Some(hour) = current_hour {
659 |                     persistence.observe_hour(hour, 1)?;
660 ~                 }
    |

error: field assignment outside of initializer for an instance created with Default::default()
   --> src\domain\pet_v2.rs:426:9
    |
426 |         state.bond = 0.77;
    |         ^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `domain::pet_state::PetStateV2 { bond: 0.77, facing: Facing::Left, ..Default::default() }` and removing relevant reassignments
   --> src\domain\pet_v2.rs:425:9
    |
425 |         let mut state = PetStateV2::default();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#field_reassign_with_default
    = note: `-D clippy::field-reassign-with-default` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::field_reassign_with_default)]`

error: field assignment outside of initializer for an instance created with Default::default()
   --> src\persistence.rs:155:9
    |
155 |         state.facing = self.facing;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `domain::pet_state::PetStateV2 { facing: self.facing, energy: self.energy.clamp(0.0, 1.0), curiosity: self.curiosity.clamp(0.0, 1.0), bond: self.bond.clamp(0.0, 1.0), sleep_pressure: self.sleep_pressure.clamp(0.0, 1.0), ..Default::default() }` and removing relevant reassignments
   --> src\persistence.rs:154:9
    |
154 |         let mut state = PetStateV2::default();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#field_reassign_with_default

error: field assignment outside of initializer for an instance created with Default::default()
    --> src\persistence.rs:1087:9
     |
1087 |         state.bond = 0.91;
     |         ^^^^^^^^^^^^^^^^^^
     |
note: consider initializing the variable with `domain::pet_state::PetStateV2 { bond: 0.91, ..Default::default() }` and removing relevant reassignments
    --> src\persistence.rs:1086:9
     |
1086 |         let mut state = PetStateV2::default();
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#field_reassign_with_default

error: field assignment outside of initializer for an instance created with Default::default()
   --> src\runtime.rs:290:9
    |
290 |         state.bond = 0.66;
    |         ^^^^^^^^^^^^^^^^^^
    |
note: consider initializing the variable with `domain::pet_state::PetStateV2 { bond: 0.66, facing: Facing::Left, ..Default::default() }` and removing relevant reassignments
   --> src\runtime.rs:289:9
    |
289 |         let mut state = PetStateV2::default();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#field_reassign_with_default

error: this `if` statement can be collapsed
   --> src\lib.rs:186:9
    |
186 | /         if let Some(runtime) = app_handle.try_state::<RuntimeHandle>() {
187 | |             if let Err(error) = runtime.close_event_input() {
188 | |                 eprintln!("Lenvu runtime input gate could not close cleanly: {error}");
189 | |             }
190 | |         }
    | |_________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#collapsible_if
help: collapse nested if block
    |
186 ~         if let Some(runtime) = app_handle.try_state::<RuntimeHandle>()
187 ~             && let Err(error) = runtime.close_event_input() {
188 |                 eprintln!("Lenvu runtime input gate could not close cleanly: {error}");
189 ~             }
    |

error: this `if` statement can be collapsed
   --> src\lib.rs:204:9
    |
204 | /         if let (Some(snapshot), Some(persistence)) = (
205 | |             frozen_snapshot,
206 | |             app_handle.try_state::<PersistenceService>(),
207 | |         ) {
...   |
214 | |         }
    | |_________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#collapsible_if
help: collapse nested if block
    |
207 ~         )
208 ~             && let Err(error) = persistence.save_and_flush(
209 |                 snapshot.state,
...
212 |                 eprintln!("Lenvu final persistence save failed: {error}");
213 ~             }
    |

error: could not compile `northpalace-my-pet` (lib) due to 20 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `northpalace-my-pet` (lib test) due to 17 previous errors
``
