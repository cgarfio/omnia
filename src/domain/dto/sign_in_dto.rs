use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Validate, Deserialize, Debug)]
pub struct SignInDto {
  #[validate(custom(function = "validate_identifier"))]
  pub identifier: String,

  #[validate(length(min = 3, message = "Must be at lease 3 characters long"))]
  pub password: String,
  // pub company :String; // ?
}

fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
  // TODO: is email

  // TODO: is username

  // TODO: is empl id ?

  // TODO: is phone number

  Ok(())
}
