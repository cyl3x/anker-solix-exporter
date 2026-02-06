use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::data::Login;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub user_id: String,
    pub auth_token: String,
    pub token_expires_at: u64,
}

impl Credentials {
    pub fn new(user_id: String, auth_token: String, token_expires_at: u64) -> Self {
        Credentials {
            user_id,
            auth_token,
            token_expires_at,
        }
    }

    pub fn expires_in(&self) -> Option<i64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|s| s.as_secs())?;

        Some(self.token_expires_at as i64 - now as i64)
    }

    pub fn load(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        match std::fs::read_to_string(path) {
            Ok(creds) => match serde_json::from_str(&creds) {
                Ok(creds) => {
                    log::info!("Loaded credentials from file");
                    Some(creds)
                }
                Err(err) => {
                    log::warn!(
                        "Failed to parse credentials from file ({path:?}): {err:?}"
                    );
                    None
                }
            },
            Err(err) => {
                log::warn!("Failed to read credentials from file ({path:?}): {err:?}");
                None
            }
        }
    }

    pub fn save(self, path: &Path) -> Self {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(err) = std::fs::create_dir_all(parent) {
                    log::warn!(
                        "Failed to create directory for credentials file ({parent:?}): {err:?}"
                    );
                }
            }
        }

        match serde_json::to_string(&self) {
            Ok(creds) => {
                if let Err(err) = std::fs::write(path, creds) {
                    log::warn!(
                        "Failed to write credentials to file ({path:?}): {err:?}"
                    );
                }
            }
            Err(err) => {
                log::warn!("Failed to serialize credentials: {err:?}");
            }
        }

        self
    }
}

impl From<Login> for Credentials {
    fn from(val: Login) -> Self {
        Credentials::new(val.user_id, val.auth_token, val.token_expires_at)
    }
}
