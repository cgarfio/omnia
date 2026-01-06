use axum::{
  Json,
  extract::FromRequest,
  response::{IntoResponse, Response},
};
use serde::{Deserialize, de::DeserializeOwned};
use validator::Validate;

// TODO: impl tryjson.validate() -> impl IntoResponse<(), json...>
pub struct TryJson<T>(pub T)
where
  T: Validate + for<'de> Deserialize<'de>;

impl<S, T> FromRequest<S> for TryJson<T>
where
  T: DeserializeOwned + Validate,
  S: Send + Sync,
{
  // TODO: impl andreturn Reply {code, message, timestamp, ...}
  type Rejection = Response;

  async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
    let Json(value) = Json::<T>::from_request(req, state).await.map_err(IntoResponse::into_response)?;

    // TODO: refactor msg into Reply struct
    if let Err(errors) = value.validate() {
      return Err(Json(errors).into_response());
    }

    Ok(TryJson(value))
  }
}
