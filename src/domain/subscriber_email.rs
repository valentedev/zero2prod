//! src/domain/subscriber_email.rs

use validator::ValidateEmail;

#[derive(Debug,Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
      if s.validate_email() {
         Ok(Self(s))
      } else {
         Err(format!("{} is not a valid subscriber email.", s))
      }
    }
}

impl AsRef<str> for SubscriberEmail {
   fn as_ref(&self) -> &str {
       &self.0
   }
}

#[cfg(test)]
mod tests {
   use super::SubscriberEmail;
   use fake::faker::internet::en::SafeEmail;
   use fake::Fake;
   use claims::assert_err;
   use rand::rngs::StdRng;
   use rand::SeedableRng;

   #[test]
   fn empty_string_is_rejected() {
      let email = "".to_string();
      assert_err!(SubscriberEmail::parse(email));
   }

   #[test]
   fn email_missing_at_symbol_is_rejected() {
      let email = "ursuladomain.com".to_string();
      assert_err!(SubscriberEmail::parse(email));
   }

   #[test]
   fn email_missing_subject_is_rejected() {
      let email = "@domain.com".to_string();
      assert_err!(SubscriberEmail::parse(email));
   }



   // #[test]
   // fn email_pass_test() {
   //    let email = "rodrigo@domain.com".to_string();
   //    assert_ok!(SubscriberEmail::parse(email));
   // }

   #[derive(Debug,Clone)]
   struct ValidEmailFixture(pub String);

   impl quickcheck::Arbitrary for ValidEmailFixture {
       fn arbitrary(g: &mut quickcheck::Gen) -> Self {
           let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
           let email = SafeEmail().fake_with_rng(&mut rng);

           Self(email)
       }
   }

   #[quickcheck_macros::quickcheck]
   fn valid_email_are_passed_successfully(valid_email: ValidEmailFixture) -> bool {
      dbg!(&valid_email.0);
      // let email = SafeEmail().fake();
      // claims::assert_ok!(SubscriberEmail::parse(email));
      SubscriberEmail::parse(valid_email.0).is_ok()
   }
}