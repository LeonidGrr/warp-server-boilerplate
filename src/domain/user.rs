use crate::domain::{user_email::UserEmail, user_name::UserName, user_password::UserPassword};

pub struct User {
    pub email: UserEmail,
    pub name: UserName,
    pub password: UserPassword,
}
