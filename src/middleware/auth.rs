use axum::{
  extract::{Request, State},
  http::{StatusCode, header::AUTHORIZATION},
  middleware::Next,
  response::Response,
};

use crate::{AppState, domain::entities::user::User};

pub async fn auth(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response, StatusCode> {
  let token = req
    .headers()
    .get(AUTHORIZATION)
    .and_then(|it| it.to_str().ok())
    .and_then(|it| it.strip_prefix("Bearer "))
    .ok_or(StatusCode::UNAUTHORIZED)?;

  // TODO: impl jwt validation

  dbg!(&token);

  let user = User {
    id: uuid::Uuid::new_v4(),
    username: "jane.doe".to_string(),
    email: "example@example.com".to_string(),
    password: "Secure123!".to_string(),
    prefix: None,
    first_name: "Jane".to_string(),
    middle_name: None,
    last_name: None,
    suffix: None,
    birthdate: chrono::NaiveDate::from_ymd_opt(1999, 6, 18).unwrap(),
    created_at: chrono::Utc::now().naive_utc(),
    updated_at: chrono::Utc::now().naive_utc(),
  };

  req.extensions_mut().insert(user);

  Ok(next.run(req).await)
}
