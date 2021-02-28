use warp::{Rejection, hyper::StatusCode};
use chrono::prelude::*;
use crate::domain::{user_email::UserEmail, user_name::UserName, user_password::UserPassword};

pub struct User {
    pub email: UserEmail,
    pub name: UserName,
    pub password: UserPassword,
    pub created_at: DateTime<chrono::Utc>,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Result<Self, Rejection> {
        let created_at = Utc::now();
        let email = UserEmail::parse(email)?;
        let name = UserName::parse(name)?;
        let password = UserPassword::parse(password)?;

        Ok(Self {
            email,
            name,
            password,
            created_at,
        })
    }
}