use std::str::FromStr;

use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Validate, Deserialize, Debug)]
pub struct SignUpDto {
  #[validate(length(min = 3, max = 20, message = "Must be between 3 and 20 (inclusive) characters long"))]
  pub username: String,
  #[validate(email(message = "Not a valid e-mail address"))]
  pub email: String,
  #[validate(length(min = 3, message = "Must be at lease 3 characters long"))]
  pub password: String,

  #[validate(length(min = 1, max = 10, message = "Must be between 1 and 10 (inclusive) characters long"))]
  pub prefix: Option<String>,
  #[validate(length(min = 3, max = 100, message = "Must be between 3 and 100 (inclusive) characters long"))]
  pub first_name: String,
  #[validate(length(min = 3, max = 100, message = "Must be between 3 and 100 (inclusive) characters long"))]
  pub middle_name: Option<String>,
  #[validate(length(min = 3, max = 100, message = "Must be between 3 and 100 (inclusive) characters long"))]
  pub last_name: Option<String>,
  #[validate(length(min = 1, max = 10, message = "Must be between 1 and 10 (inclusive) characters long"))]
  pub suffix: Option<String>,

  #[validate(custom(function = "validate_birthdate"))]
  pub birthdate: String,
}

fn validate_birthdate(birthdate: &str) -> Result<(), ValidationError> {
  chrono::NaiveDate::from_str(birthdate)
    .map(|_| Ok(()))
    .unwrap_or(Err(ValidationError::new("ISO 8601 calendar date without timezone is expected")))
}
