use std::str::FromStr;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
  AppState,
  domain::{
    dto::{sign_in_dto::SignInDto, sign_up_dto::SignUpDto},
    entities::user::User,
  },
  middleware::tryjson::TryJson,
};

pub async fn sign_up(State(state): State<AppState>, TryJson(payload): TryJson<SignUpDto>) -> impl IntoResponse {
  let mut users = state.users.lock().await;

  if users.iter().find(|it| it.username == payload.username).is_some() {
    return (StatusCode::CONFLICT, Json(json!({"message":"Username already taken"})));
  }

  if users.iter().find(|it| it.email == payload.email).is_some() {
    return (StatusCode::CONFLICT, Json(json!({"message":"E-Mail already taken"})));
  }

  dbg!(&payload);

  let user = User {
    id: uuid::Uuid::new_v4(),
    username: payload.username,
    email: payload.email,
    password: payload.password,
    prefix: payload.prefix,
    first_name: payload.first_name,
    middle_name: payload.middle_name,
    last_name: payload.last_name,
    suffix: payload.suffix,
    birthdate: chrono::NaiveDate::from_str(&payload.birthdate).unwrap(),
    created_at: chrono::Utc::now().naive_utc(),
    updated_at: chrono::Utc::now().naive_utc(),
  };

  users.push(user.clone());

  (StatusCode::CREATED, Json(json!(user)))
}

pub async fn sign_in(State(state): State<AppState>, TryJson(payload): TryJson<SignInDto>) -> impl IntoResponse {
  // TODO: remove identifier and user username | email | phone ; if not users can block other users if there are 2 or more similar users
  let users = state.users.lock().await;

  match users
    .iter()
    .find(|it| (it.username == payload.identifier || it.email == payload.identifier) && it.password == payload.password)
  {
    Some(it) => (StatusCode::OK, Json(it)).into_response(),
    None => StatusCode::UNAUTHORIZED.into_response(),
  }
}
