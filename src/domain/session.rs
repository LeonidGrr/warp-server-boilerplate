use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionPool(pub HashMap<String, Session>);

impl SessionPool {
    pub fn init() -> Arc<tokio::sync::Mutex<SessionPool>> {
        Arc::new(Mutex::new(Self(HashMap::new())))
    }

    #[tracing::instrument(name = "Registering new session.")]
    pub fn register_session(&mut self, name: &String) -> Session {
        let mut session = HashMap::new();
        session.insert("name".to_string(), name.to_string());
        let session_id = Uuid::new_v4().to_simple().to_string();
        session.insert("cookie".to_string(), session_id.clone());
        let expire_date: DateTime<Utc> = Utc::now() + Duration::days(1);
        let session = Session(session, expire_date);
        self.0.insert(session_id.clone(), session.clone());

        tracing::info!("New session succefully registered: {:?}", session);
        session
    }

    // #[tracing::instrument(name = "Validating session.")]
    // pub fn get_session(&self, session: &Session) -> Session {
    //     let Session { id } = session;

    //     if let Some(session) = self.0.get(id) {
    //         return
    //     }
    // }
}

#[derive(Debug, Clone)]
pub struct Session(pub HashMap<String, String>, DateTime<Utc>);

impl Session {
    pub fn get_cookie_header(&self) -> String {
        format!(
            "session={:?}; Max-Age=86400; Expires={}; SameSite=Strict; HttpOpnly; Secure=true",
            self.0, self.1
        )
    }
}
