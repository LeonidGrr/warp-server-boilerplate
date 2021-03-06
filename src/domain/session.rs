use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct SessionPool(pub HashMap<String, Session>);

impl SessionPool {
    pub fn init() -> Arc<tokio::sync::Mutex<SessionPool>> {
        Arc::new(Mutex::new(Self(HashMap::new())))
    }

    #[tracing::instrument(name = "Registering new session.")]
    pub fn register_session(&mut self, name: &String) -> String {
        let session_id = Uuid::new_v4().to_simple().to_string();
        let expire_date: DateTime<Utc> = Utc::now() + Duration::days(1);
        let session = Session(session_id.clone(), expire_date);
        self.0.insert(session_id.clone(), session.clone());

        tracing::info!("New session succefully registered: {:?}", session);
        format!(
            "session={}; Max-Age=86400; Expires={}; SameSite=Strict; HttpOpnly; Secure=true",
            session.0, session.1
        )
    }

    #[tracing::instrument(name = "Stop session.")]
    pub fn stop_session(&mut self, session_id: String) {
        match self.0.remove(&session_id) {
            Some(session) => {
                tracing::info!("Session {:#?}, successfully stopped.", session)
            }
            None => tracing::info!("Session with id: {}, not exist.", session_id),
        }
    }
    // #[tracing::instrument(name = "Validating session.")]
    // pub fn get_session(&self, session: &Session) -> Session {
    //     let Session { id } = session;

    //     if let Some(session) = self.0.get(id) {
    //         return
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session(pub String, pub DateTime<Utc>);
