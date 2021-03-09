use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FailedLoginAttempts {
    pub failed_at: DateTime<Utc>,
    pub count: usize,
}

#[derive(Debug)]
pub struct LoginThrottling(pub HashMap<String, FailedLoginAttempts>);

impl LoginThrottling {
    pub fn init() -> Arc<tokio::sync::Mutex<LoginThrottling>> {
        Arc::new(Mutex::new(Self(HashMap::new())))
    }

    pub fn register_failed_login_attempt(&mut self, user_name: &str) {
        if let Some(failed_attempts) = self.0.get_mut(user_name) {
            failed_attempts.count += 1;
            failed_attempts.failed_at = Utc::now();
        } else {
            let login_attempt = FailedLoginAttempts {
                failed_at: Utc::now(),
                count: 1,
            };
            self.0.insert(user_name.to_string(), login_attempt);
        }
    }

    pub fn is_login_allowed(&self, user_name: &str) -> bool {
        if let Some(failed_attempts) = self.0.get(user_name) {
            if failed_attempts.count > 5
            && failed_attempts.failed_at + Duration::minutes(1) > Utc::now() {
                return false;
            } 
        }
        true
    }

    pub fn reset_login_attempts(&mut self, user_name: &str) {
        if let Some(failed_attempts) = self.0.get_mut(user_name) {
            failed_attempts.count = 0;
        }
    }
}
