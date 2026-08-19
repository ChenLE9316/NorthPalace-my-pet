# Pre-AI Finish Diagnostics

- Source commit: ee23fc085783fdeebb122a1b374c726d4eb742c8
- patch: success
- npm ci: success
- tracked: success
- secrets: success
- target PowerShell parse: failure
- Svelte TS6: success
- Svelte TS7: success
- frontend: success
- fmt: success
- fmt check: success
- Cargo lock: success
- Clippy: failure
- tests: success

## Patch
``text
Prepared simplified base + stable hardening patch chain.
Pre-AI finish patch applied.
Current-state README semantics finalized.
Stable pre-AI semantic hardening applied.
Legacy follow-up disabled; stable hardening already applied by base patch.
``

## Svelte TS6
``text

> northpalace-my-pet@0.1.0 check:svelte
> svelte-check --fail-on-warnings --config ./vite.config.ts --tsconfig ./tsconfig.json

Loading svelte-check in workspace: d:\a\NorthPalace-my-pet\NorthPalace-my-pet
Getting Svelte diagnostics...

[32msvelte-check found 0 errors and 0 warnings
[39m
``

## Svelte TS7
``text

> northpalace-my-pet@0.1.0 check:svelte:tsgo
> svelte-check --tsgo --fail-on-warnings --config ./vite.config.ts --tsconfig ./tsconfig.json

Loading svelte-check in workspace: d:\a\NorthPalace-my-pet\NorthPalace-my-pet
Getting Svelte diagnostics...

[32msvelte-check found 0 errors and 0 warnings
[39m
``

## Frontend
``text

> northpalace-my-pet@0.1.0 build
> npm run validate:assets && vite build


> northpalace-my-pet@0.1.0 validate:assets
> node scripts/validate-lenvu-assets.mjs

[Lenvu asset contract] OK — 14 animation profiles, 1 reference asset(s), sourcePixels=measured+remapped, candidate=source_normalization_staging_ready_candidate_artwork_pending, productionReady=false
6:34:19 AM [vite-plugin-svelte] no Svelte config found at D:/a/NorthPalace-my-pet/NorthPalace-my-pet - using default configuration.
[36mvite v8.2.0 [32mbuilding client environment for production...[36m[39m
[2K
transforming...✓ 841 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                                     1.15 kB │ gzip:  0.48 kB
dist/assets/index-BvdaOVeU.css                     12.55 kB │ gzip:  2.96 kB
dist/assets/webworkerAll-zs0gkWkb.js                0.05 kB │ gzip:  0.06 kB
dist/assets/getTextureBatchBindGroup-CIgez0gS.js    0.40 kB │ gzip:  0.31 kB │ map:     1.78 kB
dist/assets/CanvasPool-DJhC69AR.js                  0.80 kB │ gzip:  0.45 kB │ map:     3.51 kB
dist/assets/canvasUtils-gItO4D4Z.js                 6.07 kB │ gzip:  2.06 kB │ map:    21.91 kB
dist/assets/BufferResource-EioRVO9d.js             10.57 kB │ gzip:  2.81 kB │ map:    25.61 kB
dist/assets/init-Bcz1Jwfn.js                       15.59 kB │ gzip:  4.91 kB │ map:    54.88 kB
dist/assets/init-C63Eqe5X.js                       24.72 kB │ gzip:  8.52 kB │ map:   105.83 kB
dist/assets/browserAll-BzSnKDMY.js                 42.63 kB │ gzip: 11.16 kB │ map:   156.60 kB
dist/assets/RenderTargetSystem-DI_S-4H-.js         71.11 kB │ gzip: 20.19 kB │ map:   262.25 kB
dist/assets/CanvasRenderer-MeLDGdTD.js             87.39 kB │ gzip: 27.41 kB │ map:   463.14 kB
dist/assets/Geometry-CUD3NC1u.js                  101.64 kB │ gzip: 31.16 kB │ map:   499.93 kB
dist/assets/index-fQh4_dWk.js                     212.84 kB │ gzip: 65.61 kB │ map: 1,101.19 kB

[32m✓ built in 686ms[39m
``

## Clippy
``text
   Compiling zerovec v0.11.7
   Compiling zerotrie v0.2.5
   Compiling thiserror-impl v1.0.69
   Compiling tinystr v0.8.4
   Compiling potential_utf v0.1.6
   Compiling darling_macro v0.23.0
   Compiling icu_collections v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling darling v0.23.0
   Compiling phf v0.13.1
   Compiling serde_json v1.0.151
   Compiling precomputed-hash v0.1.1
   Compiling getrandom v0.4.3
   Compiling windows_x86_64_msvc v0.52.6
   Compiling find-msvc-tools v0.1.11
   Compiling icu_provider v2.3.0
   Compiling shlex v2.0.1
   Compiling anyhow v1.0.104
   Compiling winnow v1.0.4
   Compiling cc v1.4.3
   Compiling icu_properties v2.3.0
   Compiling toml_parser v1.1.3+spec-1.1.0
   Compiling icu_normalizer v2.3.0
   Compiling serde_with_macros v3.22.0
   Compiling windows-implement v0.60.2
   Compiling windows-interface v0.59.3
   Compiling web_atoms v0.2.6
   Compiling parking_lot v0.12.5
   Compiling serde_spanned v1.1.1
   Compiling windows-sys v0.61.2
   Compiling toml_writer v1.1.2+spec-1.1.0
   Compiling ctor-proc-macro v0.0.7
   Compiling string_cache v0.9.0
   Compiling idna_adapter v1.2.2
   Compiling semver v1.0.28
   Compiling version_check v0.9.5
   Compiling autocfg v1.5.1
   Compiling dtoa v1.0.11
   Compiling libc v0.2.189
   Compiling log v0.4.33
   Compiling percent-encoding v2.3.2
   Compiling indexmap v1.9.3
   Compiling form_urlencoded v1.2.2
   Compiling dtoa-short v0.3.5
   Compiling uuid v1.24.1
   Compiling idna v1.1.0
   Compiling derive_more-impl v2.1.1
   Compiling cssparser-macros v0.6.1
   Compiling tendril v0.5.1
   Compiling selectors v0.36.1
   Compiling aho-corasick v1.1.5
   Compiling toml_datetime v1.1.1+spec-1.1.0
   Compiling alloc-no-stdlib v2.0.4
   Compiling camino v1.2.5
   Compiling regex-syntax v0.8.11
    Checking windows-link v0.1.3
   Compiling unic-common v0.9.0
   Compiling unic-char-range v0.9.0
   Compiling byteorder v1.5.0
   Compiling unic-char-property v0.9.0
   Compiling unic-ucd-version v0.9.0
   Compiling derive_more v2.1.1
   Compiling alloc-stdlib v0.2.4
   Compiling toml v1.1.4+spec-1.1.0
   Compiling markup5ever v0.38.0
   Compiling cssparser v0.36.0
   Compiling url v2.5.8
   Compiling winapi-util v0.1.11
   Compiling regex-automata v0.4.18
   Compiling serde_derive_internals v0.29.1
   Compiling servo_arc v0.4.3
   Compiling fnv v1.0.7
   Compiling bit-vec v0.8.0
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.12.3
   Compiling hashbrown v0.17.1
   Compiling rustc-hash v2.1.3
   Compiling deranged v0.5.8
   Compiling bitflags v2.13.1
   Compiling schemars v0.8.22
   Compiling powerfmt v0.2.0
   Compiling regex v1.13.1
   Compiling time v0.3.55
   Compiling indexmap v2.14.0
   Compiling schemars_derive v0.8.22
   Compiling bit-set v0.8.0
   Compiling cfb v0.7.3
   Compiling same-file v1.0.6
   Compiling html5ever v0.38.0
   Compiling brotli-decompressor v5.0.3
   Compiling unic-ucd-ident v0.9.0
   Compiling jsonptr v0.6.3
   Compiling cargo-platform v0.1.9
   Compiling quick-xml v0.41.0
   Compiling base64 v0.22.1
   Compiling bytes v1.12.1
   Compiling dyn-clone v1.0.20
   Compiling foldhash v0.2.0
   Compiling dunce v1.0.5
   Compiling http v1.5.0
   Compiling plist v1.10.0
   Compiling dom_query v0.27.0
   Compiling cargo_metadata v0.19.2
   Compiling json-patch v3.0.1
   Compiling brotli v8.0.4
   Compiling urlpattern v0.3.0
   Compiling walkdir v2.5.0
   Compiling serde-untagged v0.1.9
   Compiling infer v0.19.0
    Checking windows-result v0.3.4
    Checking windows-strings v0.4.2
   Compiling ctor v0.8.0
   Compiling serde_with v3.22.0
   Compiling vswhom-sys v0.1.3
   Compiling glob v0.3.4
   Compiling tauri-utils v2.9.3
   Compiling windows-targets v0.52.6
    Checking windows-core v0.61.2
   Compiling windows-sys v0.59.0
   Compiling vswhom v0.1.0
    Checking windows-threading v0.1.0
   Compiling generic-array v0.14.7
   Compiling rustc_version v0.4.1
   Compiling toml_datetime v0.7.5+spec-1.1.0
   Compiling winnow v0.7.15
   Compiling option-ext v0.2.0
   Compiling winreg v0.55.0
   Compiling embed-resource v3.0.11
   Compiling dirs-sys v0.5.0
    Checking windows-future v0.2.1
    Checking windows-collections v0.2.0
    Checking windows-numerics v0.2.0
   Compiling time-macros v0.2.32
   Compiling typenum v1.20.1
   Compiling toml v0.9.12+spec-1.1.0
   Compiling crc32fast v1.5.0
   Compiling heck v0.5.0
   Compiling simd-adler32 v0.3.10
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
    Checking unicode-segmentation v1.13.3
    Checking once_cell v1.21.4
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
    Checking windows-result v0.4.1
    Checking windows-strings v0.5.1
   Compiling winapi v0.3.9
   Compiling cpufeatures v0.2.17
   Compiling tauri-runtime v2.11.3
   Compiling wry v0.55.1
    Checking tracing-core v0.1.36
   Compiling sha2 v0.10.9
    Checking tracing v0.1.44
    Checking windows-core v0.62.2
   Compiling ico v0.5.0
    Checking keyboard-types v0.7.0
   Compiling tauri-plugin v2.6.3
   Compiling winreg v0.10.1
   Compiling pkg-config v0.3.34
   Compiling vcpkg v0.2.15
   Compiling tauri-runtime-wry v2.11.4
   Compiling getrandom v0.3.4
   Compiling libsqlite3-sys v0.38.2
   Compiling tauri-plugin-autostart v2.5.1
    Checking muda v0.19.3
   Compiling tauri-codegen v2.6.3
    Checking softbuffer v0.4.8
   Compiling serialize-to-javascript-impl v0.1.2
    Checking serialize-to-javascript v0.1.2
    Checking window-vibrancy v0.6.0
    Checking tray-icon v0.24.2
   Compiling tauri-macros v2.6.3
    Checking tokio v1.53.1
   Compiling serde_repr v0.1.21
    Checking windows-threading v0.2.1
    Checking mime v0.3.17
    Checking windows-future v0.3.2
    Checking auto-launch v0.5.0
    Checking windows-numerics v0.3.1
    Checking windows-collections v0.3.2
   Compiling northpalace-my-pet v0.1.0 (D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri)
    Checking fallible-iterator v0.3.0
    Checking fallible-streaming-iterator v0.1.9
    Checking windows v0.62.2
    Checking rusqlite v0.40.2
    Checking tao v0.35.3
    Checking webview2-com v0.38.2
error: method `as_str` is never used
  --> src\domain\memory.rs:24:12
   |
23 | impl MemoryOrigin {
   | ----------------- method in this implementation
24 |     pub fn as_str(self) -> &'static str {
   |            ^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`

error: could not compile `northpalace-my-pet` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
``

## Tests
``text
   Compiling serde_with v3.22.0
   Compiling windows-core v0.61.2
   Compiling option-ext v0.2.0
   Compiling vswhom-sys v0.1.3
   Compiling windows-sys v0.59.0
   Compiling vswhom v0.1.0
   Compiling tauri-utils v2.9.3
   Compiling windows-threading v0.1.0
   Compiling toml v0.9.12+spec-1.1.0
   Compiling winreg v0.55.0
   Compiling embed-resource v3.0.11
   Compiling windows-future v0.2.1
   Compiling dirs-sys v0.5.0
   Compiling windows-numerics v0.2.0
   Compiling windows-collections v0.2.0
   Compiling time-macros v0.2.32
   Compiling heck v0.5.0
   Compiling percent-encoding v2.3.2
   Compiling windows v0.61.3
   Compiling dirs v6.0.0
   Compiling tauri-winres v0.3.6
   Compiling cargo_toml v0.22.3
   Compiling dpi v0.1.2
   Compiling bitflags v2.13.1
   Compiling webview2-com-sys v0.38.2
   Compiling raw-window-handle v0.6.2
   Compiling tauri-build v2.6.3
   Compiling crossbeam-utils v0.8.22
   Compiling crc32fast v1.5.0
   Compiling unicode-segmentation v1.13.3
   Compiling once_cell v1.21.4
   Compiling crossbeam-channel v0.5.16
   Compiling flate2 v1.1.9
   Compiling windows-version v0.1.7
   Compiling pin-project-lite v0.2.17
   Compiling png v0.17.16
   Compiling cookie v0.18.2
   Compiling tauri v2.11.5
   Compiling windows-strings v0.5.1
   Compiling windows-result v0.4.1
   Compiling semver v1.0.28
   Compiling tracing-core v0.1.36
   Compiling sha2 v0.10.9
   Compiling tracing v0.1.44
   Compiling windows-core v0.62.2
   Compiling ico v0.5.0
   Compiling keyboard-types v0.7.0
   Compiling tauri-plugin v2.6.3
   Compiling tauri-plugin-autostart v2.5.1
   Compiling muda v0.19.3
   Compiling tauri-codegen v2.6.3
   Compiling softbuffer v0.4.8
   Compiling winapi v0.3.9
   Compiling serialize-to-javascript v0.1.2
   Compiling winreg v0.10.1
   Compiling tauri-macros v2.6.3
   Compiling window-vibrancy v0.6.0
   Compiling tray-icon v0.24.2
   Compiling getrandom v0.3.4
   Compiling tokio v1.53.1
   Compiling windows-threading v0.2.1
   Compiling mime v0.3.17
   Compiling windows-future v0.3.2
   Compiling libsqlite3-sys v0.38.2
   Compiling auto-launch v0.5.0
   Compiling windows-collections v0.3.2
   Compiling windows-numerics v0.3.1
   Compiling northpalace-my-pet v0.1.0 (D:\a\NorthPalace-my-pet\NorthPalace-my-pet\src-tauri)
   Compiling fallible-iterator v0.3.0
   Compiling fallible-streaming-iterator v0.1.9
   Compiling rusqlite v0.40.2
   Compiling windows v0.62.2
   Compiling tao v0.35.3
   Compiling webview2-com v0.38.2
   Compiling tauri-runtime v2.11.3
   Compiling wry v0.55.1
   Compiling tauri-runtime-wry v2.11.4
warning: method `as_str` is never used
  --> src\domain\memory.rs:24:12
   |
23 | impl MemoryOrigin {
   | ----------------- method in this implementation
24 |     pub fn as_str(self) -> &'static str {
   |            ^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `northpalace-my-pet` (lib) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2m 08s
     Running unittests src\lib.rs (src-tauri\target\debug\deps\northpalace_my_pet_lib-514dacbf7ee52a38.exe)

running 73 tests
test domain::personality::tests::decision_is_deterministic_for_same_context_and_index ... ok
test domain::memory::tests::memory_origin_storage_round_trip_is_stable ... ok
test domain::personality::tests::curiosity_increases_explore_weight ... ok
test domain::memory::tests::memory_kind_storage_round_trip_is_stable ... ok
test domain::personality::tests::decision_waits_for_interval ... ok
test domain::personality::tests::long_user_idle_is_left_to_rest_policy ... ok
test domain::personality::tests::night_suppresses_explore_weight ... ok
test domain::pet_v2::tests::constructor_accepts_persistent_initial_state ... ok
test domain::pet_v2::tests::dropping_returns_to_stable_ambient_posture ... ok
test domain::pet_v2::tests::facing_is_domain_state ... ok
test domain::pet_v2::tests::focus_guard_blocks_personality_ambient_actions ... ok
test domain::pet_v2::tests::focus_guard_is_restored_after_drop ... ok
test domain::pet_v2::tests::focus_mode_persists_after_entry_behavior ... ok
test domain::pet_v2::tests::personality_selector_eventually_explores_during_active_ambient_time ... ok
test domain::pet_v2::tests::pet_reaction_survives_multiple_ticks ... ok
test domain::pet_v2::tests::picked_up_posture_survives_runtime_ticks ... ok
test domain::pet_v2::tests::sleeping_recovers_energy ... ok
test memory_admin::tests::input_validation_rejects_blank_or_invalid_importance ... ok
test memory_evaluator::tests::automatic_evaluator_never_overwrites_manual_memory ... ok
test history_admin::tests::activity_list_is_newest_first ... ok
test memory_evaluator::tests::repeated_activity_becomes_a_candidate_only_after_evidence_threshold ... ok
test memory_evaluator::tests::evaluator_stores_then_merges_automatic_duplicate ... ok
test memory_admin::tests::create_update_search_and_delete_memory ... ok
test history_admin::tests::activity_history_includes_relationship_provenance ... ok
test persistence::tests::journal_ignores_high_frequency_events ... ok
test persistence::tests::loaded_state_resets_transient_runtime_fields ... ok
test persistence::tests::migration_from_v2_adds_durable_manual_origin ... ok
test persistence::tests::migration_from_v1_keeps_pet_state_and_adds_v2_tables ... ok
test persistence::tests::schema_and_pet_state_round_trip ... ok
test persistence::tests::v2_schema_supports_relationship_memory_fts_and_rhythm ... ok
test platform::windows::accessibility::tests::accessibility_bounds_preserve_negative_monitor_coordinates ... ok
test platform::windows::accessibility::tests::invalid_accessibility_bounds_are_not_exposed ... ok
test platform::windows::active_window::tests::invalid_rect_is_not_exposed_as_context ... ok
test platform::windows::active_window::tests::rect_to_bounds_preserves_negative_monitor_coordinates ... ok
test platform::windows::clock::tests::hour_normalization_never_exceeds_domain_range ... ok
test platform::windows::cursor_hit_test::tests::ellipse_contains_center ... ok
test platform::windows::cursor_hit_test::tests::rect_validation_rejects_overflow ... ok
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
test privacy::tests::corrupt_rules_keep_service_fail_closed ... ok
test privacy::tests::default_service_blocks_identity_and_accessibility_until_installed ... ok
test privacy::tests::exclusions_and_accessibility_capability_persist_and_reload ... ok
test privacy::tests::legacy_rules_without_capability_keep_accessibility_off ... ok
test privacy::tests::missing_rules_file_installs_with_accessibility_disabled ... ok
test privacy::tests::replacement_leaves_no_temp_file_after_success ... ok
test runtime::tests::managed_runtime_freezes_after_runtime_phase_shutdown ... ok
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
test persistence::tests::final_save_waits_for_worker_acknowledgement ... ok
test persistence::tests::admin_queries_share_the_persistence_worker ... ok
test worker::tests::supervisor_joins_cooperative_worker ... ok
test worker::tests::phased_shutdown_leaves_later_phase_running ... ok
test worker::tests::supervisor_records_worker_errors ... ok
test persistence::tests::online_backup_is_serialized_through_persistence_worker ... ok
test worker::tests::detached_health_remains_sticky_after_worker_returns ... ok

test result: ok. 73 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s

     Running unittests src\main.rs (src-tauri\target\debug\deps\northpalace_my_pet-b11bf1b3f410032b.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests northpalace_my_pet_lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

``
