use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Session(pub HashMap<String, String>, DateTime<Utc>);

impl Session {
    pub fn new(name: &String) -> Self {
        let mut session = HashMap::new();
        session.insert("name".to_string(), name.to_string());
        let cookie = Uuid::new_v4().to_simple().to_string();
        session.insert("cookie".to_string(), cookie);
        let expire_date: DateTime<Utc> = Utc::now() + Duration::days(1);

        Session(session, expire_date)
    }

    pub fn get_cookie_header(&self) -> String {
        format!(
            "session={:?}; Max-Age=86400; Expires={}; SameSite=Strict; HttpOpnly; Secure=true",
            self.0, self.1
        )
    }
}
