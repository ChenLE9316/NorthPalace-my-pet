# Rust Quality Repair Diagnostics

- Source commit: c99d52546a1c30223158d7c43b8a165e636221c3
- Recorded at (UTC): 2026-08-19T04:56:06.8792537Z
- patch: success
- rustfmt: success
- rustfmt check: success
- cargo lock: success
- Clippy: failure
- tests: success
- npm ci: success
- frontend build: success
- tracked-data guard: success

## Repair step
``text
not run or no output
``

## rustfmt check
``text
not run or no output
``

## Clippy
``text
   Compiling unic-ucd-version v0.9.0
   Compiling alloc-stdlib v0.2.4
   Compiling unic-char-property v0.9.0
   Compiling derive_more v2.1.1
   Compiling toml v1.1.4+spec-1.1.0
   Compiling markup5ever v0.38.0
   Compiling cssparser v0.36.0
   Compiling regex-automata v0.4.18
   Compiling url v2.5.8
   Compiling winapi-util v0.1.11
   Compiling serde_derive_internals v0.29.1
   Compiling servo_arc v0.4.3
   Compiling bit-vec v0.8.0
   Compiling schemars v0.8.22
   Compiling hashbrown v0.17.1
   Compiling deranged v0.5.8
   Compiling equivalent v1.0.2
   Compiling rustc-hash v2.1.3
   Compiling fnv v1.0.7
   Compiling bitflags v2.13.1
   Compiling powerfmt v0.2.0
   Compiling hashbrown v0.12.3
   Compiling regex v1.13.1
   Compiling time v0.3.55
   Compiling cfb v0.7.3
   Compiling indexmap v2.14.0
   Compiling schemars_derive v0.8.22
   Compiling bit-set v0.8.0
   Compiling same-file v1.0.6
   Compiling html5ever v0.38.0
   Compiling unic-ucd-ident v0.9.0
   Compiling brotli-decompressor v5.0.3
   Compiling jsonptr v0.6.3
   Compiling cargo-platform v0.1.9
   Compiling quick-xml v0.41.0
   Compiling base64 v0.22.1
   Compiling dyn-clone v1.0.20
   Compiling bytes v1.12.1
   Compiling dunce v1.0.5
   Compiling foldhash v0.2.0
   Compiling plist v1.10.0
   Compiling dom_query v0.27.0
   Compiling http v1.5.0
   Compiling cargo_metadata v0.19.2
   Compiling json-patch v3.0.1
   Compiling brotli v8.0.4
   Compiling urlpattern v0.3.0
   Compiling walkdir v2.5.0
   Compiling serde-untagged v0.1.9
   Compiling infer v0.19.0
    Checking windows-strings v0.4.2
    Checking windows-result v0.3.4
   Compiling ctor v0.8.0
   Compiling serde_with v3.22.0
   Compiling vswhom-sys v0.1.3
   Compiling glob v0.3.4
   Compiling windows-targets v0.52.6
   Compiling tauri-utils v2.9.3
    Checking windows-core v0.61.2
   Compiling windows-sys v0.59.0
   Compiling vswhom v0.1.0
    Checking windows-threading v0.1.0
   Compiling generic-array v0.14.7
   Compiling rustc_version v0.4.1
   Compiling toml_datetime v0.7.5+spec-1.1.0
   Compiling option-ext v0.2.0
   Compiling winnow v0.7.15
   Compiling winreg v0.55.0
   Compiling dirs-sys v0.5.0
   Compiling embed-resource v3.0.11
    Checking windows-future v0.2.1
    Checking windows-collections v0.2.0
    Checking windows-numerics v0.2.0
   Compiling time-macros v0.2.32
   Compiling heck v0.5.0
   Compiling toml v0.9.12+spec-1.1.0
   Compiling typenum v1.20.1
   Compiling simd-adler32 v0.3.10
   Compiling crc32fast v1.5.0
   Compiling crossbeam-utils v0.8.22
   Compiling cargo_toml v0.22.3
    Checking windows v0.61.3
   Compiling tauri-winres v0.3.6
   Compiling dirs v6.0.0
    Checking dpi v0.1.2
   Compiling webview2-com-sys v0.38.2
    Checking raw-window-handle v0.6.2
   Compiling adler2 v2.0.1
   Compiling miniz_oxide v0.8.9
   Compiling tauri-build v2.6.3
   Compiling cookie v0.18.2
    Checking once_cell v1.21.4
    Checking unicode-segmentation v1.13.3
    Checking crossbeam-channel v0.5.16
   Compiling flate2 v1.1.9
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling fdeflate v0.3.7
   Compiling webview2-com-macros v0.8.1
    Checking windows-version v0.1.7
    Checking pin-project-lite v0.2.17
   Compiling bitflags v1.3.2
   Compiling png v0.17.16
   Compiling digest v0.10.7
   Compiling tauri v2.11.5
    Checking windows-strings v0.5.1
    Checking windows-result v0.4.1
   Compiling cpufeatures v0.2.17
    Checking tracing-core v0.1.36
   Compiling wry v0.55.1
   Compiling tauri-runtime v2.11.3
   Compiling winapi v0.3.9
    Checking tracing v0.1.44
   Compiling sha2 v0.10.9
    Checking windows-core v0.62.2
   Compiling ico v0.5.0
    Checking keyboard-types v0.7.0
   Compiling tauri-plugin v2.6.3
   Compiling vcpkg v0.2.15
   Compiling winreg v0.10.1
   Compiling tauri-runtime-wry v2.11.4
   Compiling getrandom v0.3.4
   Compiling pkg-config v0.3.34
   Compiling tauri-plugin-autostart v2.5.1
   Compiling libsqlite3-sys v0.38.2
    Checking muda v0.19.3
   Compiling tauri-codegen v2.6.3
    Checking softbuffer v0.4.8
   Compiling serialize-to-javascript-impl v0.1.2
   Compiling tauri-macros v2.6.3
    Checking serialize-to-javascript v0.1.2
    Checking window-vibrancy v0.6.0
    Checking tray-icon v0.24.2
    Checking tokio v1.53.1
   Compiling serde_repr v0.1.21
    Checking windows-threading v0.2.1
    Checking mime v0.3.17
    Checking windows-future v0.3.2
    Checking auto-launch v0.5.0
    Checking windows-numerics v0.3.1
    Checking windows-collections v0.3.2
   Compiling northpalace-my-pet v0.1.0 (D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri)
    Checking fallible-streaming-iterator v0.1.9
    Checking fallible-iterator v0.3.0
    Checking windows v0.62.2
    Checking rusqlite v0.40.2
    Checking tao v0.35.3
    Checking webview2-com v0.38.2
error: unused import: `Facing`
 --> src\domain\pet_v2.rs:5:37
  |
5 |     pet_state::{Attention, Emotion, Facing, Locomotion, PetMode, PetStateV2, Posture},
  |                                     ^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
  = help: to override `-D warnings` add `#[allow(unused_imports)]`

error: variants `Run` and `Jump` are never constructed
 --> src\domain\pet_state.rs:8:5
  |
5 | pub enum Locomotion {
  |          ---------- variants in this enum
...
8 |     Run,
  |     ^^^
9 |     Jump,
  |     ^^^^
  |
  = note: `Locomotion` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
  = note: `-D dead-code` implied by `-D warnings`
  = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`

error: variants `Shy` and `Concerned` are never constructed
  --> src\domain\pet_state.rs:47:5
   |
43 | pub enum Emotion {
   |          ------- variants in this enum
...
47 |     Shy,
   |     ^^^
48 |     Concerned,
   |     ^^^^^^^^^
   |
   = note: `Emotion` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

error: variants `Listening`, `Thinking`, `Speaking`, and `Remembering` are never constructed
  --> src\domain\pet_state.rs:66:5
   |
64 | pub enum CognitionState {
   |          -------------- variants in this enum
65 |     Idle,
66 |     Listening,
   |     ^^^^^^^^^
67 |     Thinking,
   |     ^^^^^^^^
68 |     Speaking,
   |     ^^^^^^^^
69 |     Remembering,
   |     ^^^^^^^^^^^
   |
   = note: `CognitionState` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

error: variants `Shy` and `Concerned` are never constructed
  --> src\domain\pet_state.rs:47:5
   |
43 | pub enum Emotion {
   |          ------- variants in this enum
...
47 |     Shy,
   |     ^^^
48 |     Concerned,
   |     ^^^^^^^^^
   |
   = note: `Emotion` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`

error: could not compile `northpalace-my-pet` (lib test) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `northpalace-my-pet` (lib) due to 4 previous errors
``

## Tests
``text

warning: variants `Listening`, `Thinking`, `Speaking`, and `Remembering` are never constructed
  --> src\domain\pet_state.rs:66:5
   |
64 | pub enum CognitionState {
   |          -------------- variants in this enum
65 |     Idle,
66 |     Listening,
   |     ^^^^^^^^^
67 |     Thinking,
   |     ^^^^^^^^
68 |     Speaking,
   |     ^^^^^^^^
69 |     Remembering,
   |     ^^^^^^^^^^^
   |
   = note: `CognitionState` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: variants `Shy` and `Concerned` are never constructed
  --> src\domain\pet_state.rs:47:5
   |
43 | pub enum Emotion {
   |          ------- variants in this enum
...
47 |     Shy,
   |     ^^^
48 |     Concerned,
   |     ^^^^^^^^^
   |
   = note: `Emotion` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `northpalace-my-pet` (lib test) generated 2 warnings (1 duplicate)
warning: `northpalace-my-pet` (lib) generated 4 warnings (run `cargo fix --lib -p northpalace-my-pet` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2m 38s
     Running unittests src\lib.rs (src-tauri\target\debug\deps\northpalace_my_pet_lib-514dacbf7ee52a38.exe)

running 67 tests
test domain::personality::tests::curiosity_increases_explore_weight ... ok
test domain::memory::tests::memory_kind_storage_round_trip_is_stable ... ok
test domain::personality::tests::decision_is_deterministic_for_same_context_and_index ... ok
test domain::personality::tests::decision_waits_for_interval ... ok
test domain::personality::tests::long_user_idle_is_left_to_rest_policy ... ok
test domain::personality::tests::night_suppresses_explore_weight ... ok
test domain::pet_v2::tests::constructor_accepts_persistent_initial_state ... ok
test domain::pet_v2::tests::facing_is_domain_state ... ok
test domain::pet_v2::tests::dropping_returns_to_stable_ambient_posture ... ok
test domain::pet_v2::tests::focus_guard_blocks_personality_ambient_actions ... ok
test domain::pet_v2::tests::focus_guard_is_restored_after_drop ... ok
test domain::pet_v2::tests::focus_mode_persists_after_entry_behavior ... ok
test domain::pet_v2::tests::personality_selector_eventually_explores_during_active_ambient_time ... ok
test domain::pet_v2::tests::pet_reaction_survives_multiple_ticks ... ok
test domain::pet_v2::tests::picked_up_posture_survives_runtime_ticks ... ok
test domain::pet_v2::tests::sleeping_recovers_energy ... ok
test memory_admin::tests::input_validation_rejects_blank_or_invalid_importance ... ok
test history_admin::tests::activity_list_is_newest_first ... ok
test history_admin::tests::activity_history_includes_relationship_provenance ... ok
test persistence::tests::journal_ignores_high_frequency_events ... ok
test memory_admin::tests::create_update_search_and_delete_memory ... ok
test persistence::tests::loaded_state_resets_transient_runtime_fields ... ok
test persistence::tests::migration_from_v1_keeps_pet_state_and_adds_v2_tables ... ok
test persistence::tests::schema_and_pet_state_round_trip ... ok
test platform::windows::accessibility::tests::accessibility_bounds_preserve_negative_monitor_coordinates ... ok
test platform::windows::accessibility::tests::invalid_accessibility_bounds_are_not_exposed ... ok
test platform::windows::active_window::tests::invalid_rect_is_not_exposed_as_context ... ok
test platform::windows::active_window::tests::rect_to_bounds_preserves_negative_monitor_coordinates ... ok
test platform::windows::clock::tests::hour_normalization_never_exceeds_domain_range ... ok
test platform::windows::cursor_hit_test::tests::ellipse_contains_center ... ok
test platform::windows::cursor_hit_test::tests::rect_validation_rejects_overflow ... ok
test persistence::tests::v2_schema_supports_relationship_memory_fts_and_rhythm ... ok
test platform::windows::idle::tests::wrapping_tick_math_matches_windows_tick_semantics ... ok
test platform::windows::motion::tests::adjacent_horizontal_monitor_is_selected ... ok
test platform::windows::motion::tests::disconnected_monitor_gap_prevents_autonomous_teleport ... ok
test platform::windows::motion::tests::initial_direction_respects_domain_facing ... ok
test platform::windows::motion::tests::motion_reverses_at_right_edge_without_transition ... ok
test platform::windows::motion::tests::only_ambient_explore_can_cross_monitors ... ok
test platform::windows::motion::tests::stationary_and_jump_do_not_translate_window ... ok
test platform::windows::motion::tests::vertically_stacked_monitor_is_not_a_horizontal_neighbor ... ok
test platform::windows::motion::tests::work_area_clamps_pet_inside_edges ... ok
test privacy::tests::app_id_normalization_is_case_insensitive_and_strips_exe ... ok
test privacy::tests::default_service_blocks_identity_and_accessibility_until_installed ... ok
test privacy::tests::corrupt_rules_keep_service_fail_closed ... ok
test privacy::tests::legacy_rules_without_capability_keep_accessibility_off ... ok
test privacy::tests::missing_rules_file_installs_with_accessibility_disabled ... ok
test privacy::tests::exclusions_and_accessibility_capability_persist_and_reload ... ok
test privacy::tests::replacement_leaves_no_temp_file_after_success ... ok
test runtime::tests::runtime_dispatches_domain_events_to_observers ... ok
test runtime::tests::runtime_publishes_snapshots_to_observer ... ok
test runtime::tests::runtime_starts_from_supplied_state ... ok
test screen_context::tests::accessibility_disabled_does_not_override_privacy_block ... ok
test screen_context::tests::accessibility_unavailable_does_not_override_privacy_block ... ok
test screen_context::tests::disabled_accessibility_survives_active_app_heartbeats ... ok
test screen_context::tests::privacy_block_clears_previous_app_identity_bounds_and_accessibility ... ok
test screen_context::tests::repeated_heartbeat_refreshes_timestamp_without_advancing_sequence ... ok
test screen_context::tests::stale_accessibility_result_for_previous_app_is_ignored ... ok
test screen_context::tests::structured_signals_include_freshness_without_screen_pixels ... ok
test screen_context::tests::switching_apps_invalidates_previous_accessibility_context_and_freshness ... ok
test worker::tests::cancellation_interrupts_wait ... ok
test worker::tests::detached_health_remains_sticky_after_worker_returns ... ok
test runtime::tests::managed_runtime_freezes_after_runtime_phase_shutdown ... ok
test worker::tests::phased_shutdown_leaves_later_phase_running ... ok
test worker::tests::supervisor_joins_cooperative_worker ... ok
test worker::tests::supervisor_records_worker_errors ... ok
test persistence::tests::final_save_waits_for_worker_acknowledgement ... ok
test persistence::tests::admin_queries_share_the_persistence_worker ... ok

test result: ok. 67 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s

     Running unittests src\main.rs (src-tauri\target\debug\deps\northpalace_my_pet-b11bf1b3f410032b.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests northpalace_my_pet_lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

``
