use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyRulesSnapshot {
    pub excluded_apps: Vec<String>,
    pub fail_closed: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredPrivacyRules {
    excluded_apps: Vec<String>,
}

#[derive(Debug)]
struct PrivacyState {
    path: Option<PathBuf>,
    excluded_apps: BTreeSet<String>,
    fail_closed: bool,
}

impl Default for PrivacyState {
    fn default() -> Self {
        Self {
            path: None,
            excluded_apps: BTreeSet::new(),
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

        let excluded_apps = load_rules(&path)?;
        let mut state = self
            .inner
            .write()
            .map_err(|_| "privacy-policy lock is poisoned".to_owned())?;
        state.path = Some(path);
        state.excluded_apps = excluded_apps;
        state.fail_closed = false;
        Ok(())
    }

    pub fn snapshot(&self) -> PrivacyRulesSnapshot {
        let Ok(state) = self.inner.read() else {
            return PrivacyRulesSnapshot {
                excluded_apps: Vec::new(),
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

    pub fn add_excluded_app(&self, app_id: &str) -> Result<PrivacyRulesSnapshot, String> {
        let normalized = normalize_app_id(app_id)?;
        self.update_rules(|rules| {
            rules.insert(normalized);
        })
    }

    pub fn remove_excluded_app(&self, app_id: &str) -> Result<PrivacyRulesSnapshot, String> {
        let normalized = normalize_app_id(app_id)?;
        self.update_rules(|rules| {
            rules.remove(&normalized);
        })
    }

    fn update_rules(
        &self,
        update: impl FnOnce(&mut BTreeSet<String>),
    ) -> Result<PrivacyRulesSnapshot, String> {
        let mut state = self
            .inner
            .write()
            .map_err(|_| "privacy-policy lock is poisoned".to_owned())?;
        if state.fail_closed {
            return Err("privacy rules are unavailable; app identity remains blocked".to_owned());
        }
        let path = state
            .path
            .clone()
            .ok_or_else(|| "privacy-rules path is unavailable".to_owned())?;

        let mut next = state.excluded_apps.clone();
        update(&mut next);
        persist_rules(&path, &next)?;
        state.excluded_apps = next;
        Ok(snapshot_from_state(&state))
    }

    fn mark_fail_closed(&self) {
        if let Ok(mut state) = self.inner.write() {
            state.fail_closed = true;
        }
    }
}

fn snapshot_from_state(state: &PrivacyState) -> PrivacyRulesSnapshot {
    PrivacyRulesSnapshot {
        excluded_apps: state.excluded_apps.iter().cloned().collect(),
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

fn load_rules(path: &Path) -> Result<BTreeSet<String>, String> {
    if !path.exists() {
        return Ok(BTreeSet::new());
    }

    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read privacy rules {}: {error}", path.display()))?;
    let stored: StoredPrivacyRules = serde_json::from_str(&content)
        .map_err(|error| format!("invalid privacy rules {}: {error}", path.display()))?;

    stored
        .excluded_apps
        .into_iter()
        .map(|app_id| normalize_app_id(&app_id))
        .collect()
}

fn persist_rules(path: &Path, rules: &BTreeSet<String>) -> Result<(), String> {
    let stored = StoredPrivacyRules {
        excluded_apps: rules.iter().cloned().collect(),
    };
    let content = serde_json::to_string_pretty(&stored)
        .map_err(|error| format!("failed to serialize privacy rules: {error}"))?;
    fs::write(path, format!("{content}\n"))
        .map_err(|error| format!("failed to write privacy rules {}: {error}", path.display()))
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
    fn default_service_blocks_identity_until_installed() {
        let privacy = PrivacyPolicyService::default();
        assert!(privacy.snapshot().fail_closed);
        assert!(privacy.is_app_excluded("explorer"));
    }

    #[test]
    fn missing_rules_file_installs_as_empty_open_policy() {
        let path = unique_test_path();
        let privacy = PrivacyPolicyService::default();
        privacy.install(path.clone()).expect("install privacy rules");
        assert!(!privacy.snapshot().fail_closed);
        assert!(!privacy.is_app_excluded("explorer"));
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    #[test]
    fn exclusions_persist_and_reload() {
        let path = unique_test_path();
        let privacy = PrivacyPolicyService::default();
        privacy.install(path.clone()).expect("install privacy rules");
        let snapshot = privacy
            .add_excluded_app("KeePassXC.EXE")
            .expect("add exclusion");
        assert_eq!(snapshot.excluded_apps, vec!["keepassxc"]);
        assert!(privacy.is_app_excluded("KEEPASSXC"));

        let reloaded = PrivacyPolicyService::default();
        reloaded.install(path.clone()).expect("reload privacy rules");
        assert!(reloaded.is_app_excluded("keepassxc.exe"));
        reloaded
            .remove_excluded_app("keepassxc")
            .expect("remove exclusion");
        assert!(!reloaded.is_app_excluded("keepassxc"));

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

        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
}
