use axum::{Router, routing::get};
use crate::api::*;

use crate::SharedState;

pub fn create_routes(state: SharedState) -> Router {
    Router::new()
        .route("/students", get(get_students).post(add_student))
        .route("/students/{id}", get(get_students).put(update_student).delete(delete_student))
        .with_state(state)
}

