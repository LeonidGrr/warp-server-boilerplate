use crate::errors::Errors;
use validator::validate_email;
use warp::{reject, Rejection};

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: &str) -> Result<UserEmail, Rejection> {
        if validate_email(s) {
            return Ok(Self(s.to_string()));
        }
        Err(reject::custom(Errors::EmailNotValid(s.to_string())))
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::UserEmail;
    use claim::assert_err;
    // use fake::faker::internet::en::SafeEmail;
    // use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    // impl quickcheck::Arbitrary for ValidEmailFixture {
    //     fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
    //         let email = SafeEmail().fake_with_rng(g);
    //         Self(email)
    //     }
    // }

    // #[quickcheck_macros::quickcheck]
    // fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
    //     UserEmail::parse(valid_email.0).is_ok()
    // }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(UserEmail::parse(&email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(UserEmail::parse(&email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(UserEmail::parse(&email));
    }
}
