use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::{middleware::auth::auth_middleware, state::AppState};

use super::handler;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/upload", post(handler::upload_file))
        .route("/", get(handler::list_files))
        .route("/:id", delete(handler::delete_file))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}
