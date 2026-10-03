use axum::{
    middleware,
    routing::{delete, get, patch},
    Router,
};

use crate::{
    middleware::{auth::auth_middleware, rbac::require_role},
    modules::user::model::Role,
    state::AppState,
};

use super::handler;

pub fn routes(state: AppState) -> Router<AppState> {
    // Routes that require auth (any role)
    let authenticated = Router::new()
        .route("/:id", get(handler::get_user))
        .route("/:id", patch(handler::update_user))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Admin-only routes: auth first, then role check
    // Note: route_layer is applied bottom-up, so auth runs before role guard
    let admin_only = Router::new()
        .route("/", get(handler::list_users))
        .route("/:id", delete(handler::delete_user))
        .route_layer(middleware::from_fn(require_role(Role::Admin)))
        .route_layer(middleware::from_fn_with_state(
            state,
            auth_middleware,
        ));

    Router::new().merge(authenticated).merge(admin_only)
}
