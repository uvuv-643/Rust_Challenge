use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    order_handler::{
        order_create_handler,
        order_index_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/orders/", get(order_index_handler))
        .route("/api/orders/", post(order_create_handler))
        .with_state(app_state)
}