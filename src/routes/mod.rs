//! Central router.
//!
//! To add a new module, import its `routes` function and nest it here.
//! The `state` parameter is forwarded to every module's `routes(state)` call
//! so that middleware factories (which need a concrete state value) can be
//! registered inside each module.

use axum::Router;

use crate::{
    modules::{auth, media, user},
    state::AppState,
};

pub fn build(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/api/auth", auth::routes::routes(state.clone()))
        .nest("/api/users", user::routes::routes(state.clone()))
        .nest("/api/media", media::routes::routes(state))
}
