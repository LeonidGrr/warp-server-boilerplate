use crate::errors::Errors;
use argon2::{self, Config};
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;
use warp::{reject, Rejection};

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

#[derive(Debug)]
pub struct UserPassword(pub String);

impl UserPassword {
    pub fn parse(password: &str) -> Result<UserPassword, Rejection> {
        let is_empty_or_whitespace = password.trim().is_empty();
        let is_too_short = password.graphemes(true).count() < 8;
        let is_too_long = password.graphemes(true).count() > 256;
        if is_empty_or_whitespace || is_too_short || is_too_long {
            return Err(reject::custom(Errors::PasswordNotValid));
        }
        let hash = Self::hash_password(password)?;

        Ok(Self(hash))
    }

    fn hash_password(password: &str) -> Result<String, Rejection> {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config {
            secret: SECRET_KEY.as_bytes(),
            ..Default::default()
        };

        let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|e| reject::custom(Errors::PasswordEncodeFailed(e)));

        hash
    }

    pub fn verify(hash: &str, password: &str) -> Result<bool, Rejection> {
        argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[])
            .map_err(|e| reject::custom(Errors::PasswordEncodeFailed(e)))
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
