use std::{
    collections::BTreeSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyRulesSnapshot {
    pub excluded_apps: Vec<String>,
    pub accessibility_context_enabled: bool,
    pub fail_closed: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredPrivacyRules {
    #[serde(default)]
    excluded_apps: Vec<String>,
    #[serde(default)]
    accessibility_context_enabled: bool,
}

#[derive(Debug)]
struct PrivacyState {
    path: Option<PathBuf>,
    excluded_apps: BTreeSet<String>,
    accessibility_context_enabled: bool,
    fail_closed: bool,
}

impl Default for PrivacyState {
    fn default() -> Self {
        Self {
            path: None,
            excluded_apps: BTreeSet::new(),
            accessibility_context_enabled: false,
            fail_closed: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct PrivacyPolicyService {
    inner: Arc<RwLock<PrivacyState>>,
}

impl PrivacyPolicyService {
    pub fn install(&self, path: PathBuf) -> Result<(), String> {
        self.mark_fail_closed();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create privacy-rules directory {}: {error}",
                    parent.display()
                )
            })?;
        }

        let stored = load_rules(&path)?;
        let excluded_apps = stored
            .excluded_apps
            .into_iter()
            .map(|app_id| normalize_app_id(&app_id))
            .collect::<Result<BTreeSet<_>, _>>()?;

        let mut state = self
            .inner
            .write()
            .map_err(|_| "privacy-policy lock is poisoned".to_owned())?;
        state.path = Some(path);
        state.excluded_apps = excluded_apps;
        state.accessibility_context_enabled = stored.accessibility_context_enabled;
        state.fail_closed = false;
        Ok(())
    }

    pub fn snapshot(&self) -> PrivacyRulesSnapshot {
        let Ok(state) = self.inner.read() else {
            return PrivacyRulesSnapshot {
                excluded_apps: Vec::new(),
                accessibility_context_enabled: false,
                fail_closed: true,
            };
        };
        snapshot_from_state(&state)
    }

    pub fn is_app_excluded(&self, app_id: &str) -> bool {
        let Ok(normalized) = normalize_app_id(app_id) else {
            return true;
        };
        let Ok(state) = self.inner.read() else {
            return true;
        };
        state.fail_closed || state.excluded_apps.contains(&normalized)
    }

    pub fn is_accessibility_context_allowed(&self, app_id: &str) -> bool {
        let Ok(normalized) = normalize_app_id(app_id) else {
            return false;
        };
        let Ok(state) = self.inner.read() else {
            return false;
        };

        !state.fail_closed
            && state.accessibility_context_enabled
            && !state.excluded_apps.contains(&normalized)
    }

    pub fn add_excluded_app(&self, app_id: &str) -> Result<PrivacyRulesSnapshot, String> {
        let normalized = normalize_app_id(app_id)?;
        self.update_state(|state| {
            state.excluded_apps.insert(normalized);
        })
    }

    pub fn remove_excluded_app(&self, app_id: &str) -> Result<PrivacyRulesSnapshot, String> {
        let normalized = normalize_app_id(app_id)?;
        self.update_state(|state| {
            state.excluded_apps.remove(&normalized);
        })
    }

    pub fn set_accessibility_context_enabled(
        &self,
        enabled: bool,
    ) -> Result<PrivacyRulesSnapshot, String> {
        self.update_state(|state| {
            state.accessibility_context_enabled = enabled;
        })
    }

    fn update_state(
        &self,
        update: impl FnOnce(&mut PrivacyState),
    ) -> Result<PrivacyRulesSnapshot, String> {
        let mut state = self
            .inner
            .write()
            .map_err(|_| "privacy-policy lock is poisoned".to_owned())?;
        if state.fail_closed {
            return Err("privacy rules are unavailable; sensitive context remains blocked".to_owned());
        }
        let path = state
            .path
            .clone()
            .ok_or_else(|| "privacy-rules path is unavailable".to_owned())?;

        let previous_excluded_apps = state.excluded_apps.clone();
        let previous_accessibility = state.accessibility_context_enabled;
        update(&mut state);

        if let Err(error) = persist_rules(&path, &state) {
            state.excluded_apps = previous_excluded_apps;
            state.accessibility_context_enabled = previous_accessibility;
            return Err(error);
        }

        Ok(snapshot_from_state(&state))
    }

    fn mark_fail_closed(&self) {
        if let Ok(mut state) = self.inner.write() {
            state.fail_closed = true;
            state.accessibility_context_enabled = false;
        }
    }
}

fn snapshot_from_state(state: &PrivacyState) -> PrivacyRulesSnapshot {
    PrivacyRulesSnapshot {
        excluded_apps: state.excluded_apps.iter().cloned().collect(),
        accessibility_context_enabled: state.accessibility_context_enabled,
        fail_closed: state.fail_closed,
    }
}

fn normalize_app_id(app_id: &str) -> Result<String, String> {
    let value = app_id.trim().to_lowercase();
    let value = value.strip_suffix(".exe").unwrap_or(&value).trim();
    if value.is_empty() {
        return Err("app id cannot be empty".to_owned());
    }
    if value.chars().count() > 128 {
        return Err("app id is too long (maximum 128 characters)".to_owned());
    }
    if value.chars().any(char::is_control) {
        return Err("app id cannot contain control characters".to_owned());
    }
    Ok(value.to_owned())
}

fn load_rules(path: &Path) -> Result<StoredPrivacyRules, String> {
    if !path.exists() {
        return Ok(StoredPrivacyRules::default());
    }

    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read privacy rules {}: {error}", path.display()))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("invalid privacy rules {}: {error}", path.display()))
}

fn persist_rules(path: &Path, state: &PrivacyState) -> Result<(), String> {
    let stored = StoredPrivacyRules {
        excluded_apps: state.excluded_apps.iter().cloned().collect(),
        accessibility_context_enabled: state.accessibility_context_enabled,
    };
    let content = serde_json::to_string_pretty(&stored)
        .map_err(|error| format!("failed to serialize privacy rules: {error}"))?;
    let temp_path = path.with_extension("json.tmp");

    let result = (|| -> Result<(), String> {
        let mut file = fs::File::create(&temp_path).map_err(|error| {
            format!(
                "failed to create temporary privacy rules {}: {error}",
                temp_path.display()
            )
        })?;
        file.write_all(format!("{content}\n").as_bytes())
            .map_err(|error| format!("failed to write temporary privacy rules: {error}"))?;
        file.sync_all()
            .map_err(|error| format!("failed to flush temporary privacy rules: {error}"))?;
        drop(file);

        replace_file(&temp_path, path)
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

#[cfg(target_os = "windows")]
fn replace_file(source: &Path, destination: &Path) -> Result<(), String> {
    use std::{iter, os::windows::ffi::OsStrExt};

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x0000_0001;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x0000_0008;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(existing_file_name: *const u16, new_file_name: *const u16, flags: u32) -> i32;
    }

    let source_wide = source
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();
    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();

    let ok = unsafe {
        MoveFileExW(
            source_wide.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if ok == 0 {
        return Err(format!(
            "failed to atomically replace privacy rules {}: {}",
            destination.display(),
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn replace_file(source: &Path, destination: &Path) -> Result<(), String> {
    fs::rename(source, destination).map_err(|error| {
        format!(
            "failed to atomically replace privacy rules {}: {error}",
            destination.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_path() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir()
            .join(format!("northpalace-privacy-{}-{nonce}", std::process::id()))
            .join("privacy-rules.json")
    }

    #[test]
    fn app_id_normalization_is_case_insensitive_and_strips_exe() {
        assert_eq!(normalize_app_id("  Discord.EXE ").unwrap(), "discord");
        assert!(normalize_app_id("   ").is_err());
    }

    #[test]
    fn default_service_blocks_identity_and_accessibility_until_installed() {
        let privacy = PrivacyPolicyService::default();
        let snapshot = privacy.snapshot();
        assert!(snapshot.fail_closed);
        assert!(!snapshot.accessibility_context_enabled);
        assert!(privacy.is_app_excluded("explorer"));
        assert!(!privacy.is_accessibility_context_allowed("explorer"));
    }

    #[test]
    fn missing_rules_file_installs_with_accessibility_disabled() {
        let path = unique_test_path();
        let privacy = PrivacyPolicyService::default();
        privacy.install(path.clone()).expect("install privacy rules");
        let snapshot = privacy.snapshot();
        assert!(!snapshot.fail_closed);
        assert!(!snapshot.accessibility_context_enabled);
        assert!(!privacy.is_app_excluded("explorer"));
        assert!(!privacy.is_accessibility_context_allowed("explorer"));
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn legacy_rules_without_capability_keep_accessibility_off() {
        let path = unique_test_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create temp privacy directory");
        }
        fs::write(&path, r#"{"excludedApps":["discord"]}"#)
            .expect("write legacy privacy rules");

        let privacy = PrivacyPolicyService::default();
        privacy.install(path.clone()).expect("load legacy rules");
        let snapshot = privacy.snapshot();
        assert_eq!(snapshot.excluded_apps, vec!["discord"]);
        assert!(!snapshot.accessibility_context_enabled);
        assert!(!privacy.is_accessibility_context_allowed("explorer"));

        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn exclusions_and_accessibility_capability_persist_and_reload() {
        let path = unique_test_path();
        let privacy = PrivacyPolicyService::default();
        privacy.install(path.clone()).expect("install privacy rules");
        privacy
            .add_excluded_app("KeePassXC.EXE")
            .expect("add exclusion");
        let snapshot = privacy
            .set_accessibility_context_enabled(true)
            .expect("enable accessibility context");
        assert_eq!(snapshot.excluded_apps, vec!["keepassxc"]);
        assert!(snapshot.accessibility_context_enabled);
        assert!(privacy.is_accessibility_context_allowed("explorer"));
        assert!(!privacy.is_accessibility_context_allowed("KEEPASSXC"));

        let reloaded = PrivacyPolicyService::default();
        reloaded.install(path.clone()).expect("reload privacy rules");
        let reloaded_snapshot = reloaded.snapshot();
        assert!(reloaded_snapshot.accessibility_context_enabled);
        assert!(!reloaded.is_accessibility_context_allowed("keepassxc.exe"));
        assert!(reloaded.is_accessibility_context_allowed("explorer"));

        reloaded
            .remove_excluded_app("keepassxc")
            .expect("remove exclusion");
        assert!(reloaded.is_accessibility_context_allowed("keepassxc"));

        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn replacement_leaves_no_temp_file_after_success() {
        let path = unique_test_path();
        let privacy = PrivacyPolicyService::default();
        privacy.install(path.clone()).expect("install privacy rules");
        privacy.add_excluded_app("code").expect("first write");
        privacy.add_excluded_app("discord").expect("replacement write");

        assert!(path.exists());
        assert!(!path.with_extension("json.tmp").exists());

        let reloaded = PrivacyPolicyService::default();
        reloaded.install(path.clone()).expect("reload replacement");
        assert_eq!(reloaded.snapshot().excluded_apps, vec!["code", "discord"]);

        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn corrupt_rules_keep_service_fail_closed() {
        let path = unique_test_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create temp privacy directory");
        }
        fs::write(&path, "not-json").expect("write corrupt privacy rules");

        let privacy = PrivacyPolicyService::default();
        assert!(privacy.install(path.clone()).is_err());
        assert!(privacy.snapshot().fail_closed);
        assert!(privacy.is_app_excluded("explorer"));
        assert!(!privacy.is_accessibility_context_allowed("explorer"));

        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
}