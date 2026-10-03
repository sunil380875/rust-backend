use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{middleware::auth::auth_middleware, state::AppState};

use super::handler;

/// Build the auth router. The auth middleware is applied as a route_layer so
/// that `/register` and `/login` remain public while `/me` is protected.
///
/// Note: `from_fn_with_state` receives the `AppState` at *request time* via
/// Axum's internal state-passing mechanism when the outer router is finalised
/// with `.with_state(state)`.  We pass the `state` parameter here so that the
/// middleware factory gets a concrete value; `routes()` is called from
/// `routes/mod.rs` which already has access to the state.
pub fn routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login));

    let protected = Router::new()
        .route("/me", get(handler::me))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware));

    Router::new().merge(public).merge(protected)
}
