#[path = "handlers/order/order.rs"] mod order_handler;
#[path = "handlers/order/order_create.rs"] mod order_handler_request;
mod router;

use std::sync::Arc;
use dotenv::dotenv;
use tokio::net::TcpListener;

use crate::router::create_router;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[path = "model/order.rs"] mod order;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();

}