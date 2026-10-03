use std::future::Future;
use std::pin::Pin;

use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::{error::AppError, modules::auth::dto::Claims, modules::user::model::Role};

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// Role-based access control middleware factory.
///
/// Returns a middleware function that can be passed to `axum::middleware::from_fn`.
///
/// # Example
/// ```rust
/// router.route_layer(middleware::from_fn(require_role(Role::Admin)))
/// ```
pub fn require_role(
    required: Role,
) -> impl Fn(Request<Body>, Next) -> BoxFuture<Result<Response, AppError>>
       + Clone
       + Send
       + 'static {
    move |req: Request<Body>, next: Next| {
        let required = required.clone();
        Box::pin(async move {
            let claims = req
                .extensions()
                .get::<Claims>()
                .ok_or_else(|| AppError::Unauthorized("Not authenticated".into()))?
                .clone();

            if claims.role != required {
                return Err(AppError::Forbidden(format!(
                    "Access denied: requires {:?} role",
                    required
                )));
            }
            Ok(next.run(req).await)
        })
    }
}
