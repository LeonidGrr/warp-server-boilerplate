use crate::errors::Errors;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{reject, Rejection};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session(pub String, pub DateTime<Utc>);

#[derive(Debug)]
pub struct SessionPool(pub HashMap<String, Session>);

impl SessionPool {
    pub fn init() -> Arc<tokio::sync::Mutex<SessionPool>> {
        Arc::new(Mutex::new(Self(HashMap::new())))
    }

    pub fn register_session(&mut self) -> String {
        let session_id = Uuid::new_v4().to_simple().to_string();
        let expire_at: DateTime<Utc> = Utc::now() + Duration::days(1);
        let session = Session(session_id.clone(), expire_at);
        self.0.insert(session_id.clone(), session.clone());
        tracing::info!("New session successfully registered: {:?}", session);
        format!(
            "session={}; Max-Age=86400; Expires={}; SameSite=Strict; HttpOpnly; Secure=true",
            session.0, session.1
        )
    }

    pub fn stop_session(&mut self, session_id: &String) -> Result<Session, Rejection> {
        if let Some(session) = self.0.remove(session_id) {
            tracing::info!("Session {:#?}, successfully stopped.", session);
            return Ok(session);
        }
        Err(reject::custom(Errors::InvalidSession))
    }

    pub fn get_session(&mut self, session_id: &String) -> Result<Session, Rejection> {
        if let Some(session) = self.0.get_mut(session_id) {
            tracing::info!("Session {:#?} retrieved.", session);
            return Ok(session.clone());
        }
        Err(reject::custom(Errors::InvalidSession))
    }

    pub fn validate_session(&mut self, session_id: &String) -> Result<bool, Rejection> {
        if let Some(session) = self.0.get_mut(session_id) {
            let expire_at = session.1;
            let is_valid = Utc::now() < expire_at;
            return Ok(is_valid);
        }
        Err(reject::custom(Errors::InvalidSession))
    }

    pub fn update_session(&mut self, session_id: &String) -> Result<String, Rejection> {
        if let Some(session) = self.0.get_mut(session_id) {
            session.1 = Utc::now() + Duration::days(1);
            tracing::info!("Session successfully updated: {:?}", session);
            return Ok(format!(
                "session={}; Max-Age=86400; Expires={}; SameSite=Strict; HttpOpnly; Secure=true",
                session.0, session.1
            ));
        }
        Err(reject::custom(Errors::InvalidSession))
    }
}
