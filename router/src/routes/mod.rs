use crate::state::AppState;
use axum::Router;

mod index;
mod user;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/index", index::router())
        .nest("/users", user::router())
        .with_state(state)
}
