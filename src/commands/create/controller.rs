use std::sync::Arc;

use axum::{
    routing::{post},
    Router,
};
use crate::commands::create::service::create;
use crate::AppState;

pub fn create_service(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/pet/", post(create))
        .with_state(app_state)
}