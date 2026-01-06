use std::{collections::HashMap, sync::Arc};

use axum::{
  Json, Router,
  extract::{Extension, State},
  routing::{get, post},
};
use serde_json::json;
use tokio::sync::Mutex;
use tower_http::normalize_path::NormalizePathLayer;

use crate::domain::entities::user::User;

mod cli;
mod domain;
mod handler;
mod middleware;

#[derive(Clone, Default)]
struct AppState {
  users: Arc<Mutex<Vec<User>>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let args = cli::Args::parse();

  let state = AppState::default();

  let public_routes = Router::new()
    .route("/", get(async || "Hello, world!"))
    .route("/auth/sign-up", post(handler::auth::sign_up))
    .route("/auth/sign-in", post(handler::auth::sign_in))
    .route(
      "/users",
      get(async |State::<AppState>(state)| {
        let users = state.users.lock().await.clone();
        Json(json!(users))
      }),
    );

  let private_routes = Router::new()
    .route("/auth/me", get(async |Extension::<User>(user)| Json(user)))
    .layer(axum::middleware::from_fn_with_state(state.clone(), middleware::auth::auth));

  let app = Router::new()
    .merge(public_routes)
    .merge(private_routes)
    .layer(NormalizePathLayer::trim_trailing_slash())
    .with_state(state);

  let listener = tokio::net::TcpListener::bind(args.socket).await?;
  println!("Server listening at http://{}", listener.local_addr()?);
  axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;

  Ok(())
}

async fn shutdown_signal() {
  let ctrl_c = async {
    tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}
