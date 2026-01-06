use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
  pub id: uuid::Uuid,
  pub username: String,
  pub email: String,
  #[serde(skip_serializing)]
  pub password: String,

  pub prefix: Option<String>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: Option<String>,
  pub suffix: Option<String>,

  pub birthdate: chrono::NaiveDate,

  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}
