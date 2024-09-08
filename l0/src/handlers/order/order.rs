use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime};
use crate::{order, AppState};

use order::Order;
use order::Delivery;
use order::Payment;
use order::Item;
use crate::order_handler_request::OrderCreate;

pub async fn order_index_handler(State(data): State<Arc<AppState>>) -> Result<String, StatusCode> {
    let query_result = sqlx::query_as!(
        Order,
        r#"
        SELECT orders.*,
        (delivery.name, delivery.phone, delivery.zip, delivery.city, delivery.address, delivery.region, delivery.email) as "delivery!: Delivery",
        (payment."transaction", payment.request_id, payment.currency, payment.provider, payment.amount, payment.payment_dt, payment.bank, payment.delivery_cost, payment.goods_total, payment.custom_fee) as "payment!: Payment",
        COALESCE(
           ARRAY_AGG(
               (C.chrt_id, C.track_number, C.price, C.rid, C.name, C.sale, C.size, C.total_price, C.nm_id, C.brand, C.status)
           ) FILTER (WHERE C.chrt_id IS NOT NULL),
           ARRAY[]::RECORD[]
        ) as "items: Vec<Item>"
        FROM orders
        LEFT JOIN delivery ON orders.order_uid = delivery.order_uid
        LEFT JOIN items C ON orders.order_uid = C.order_uid
        LEFT JOIN payment ON orders.order_uid = payment.order_uid
        GROUP BY orders.order_uid, delivery.id, payment.order_uid
        "#,
    )
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let json_parse_result = serde_json::to_string(&query_result.unwrap());

    if json_parse_result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    Ok(json_parse_result.unwrap())

}

/**
    TODO: validation with 400 status instead of 500 error
    TODO: Payment <-> Order and Delivery <-> Order many to many?
*/
pub async fn order_create_handler(State(data): State<Arc<AppState>>, Json(payload): Json<OrderCreate>) -> Result<String, (StatusCode, String)>  {

    let order_uid = payload.order_uid;
    let mut tx = data.db.begin().await.expect("Cannot start transaction");

    let delivery = payload.delivery.expect("Incorrect Delivery Given");
    let result : Result<_, _> = sqlx::query("insert into delivery (order_uid, name, phone, zip, city, address, region, email) values ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(order_uid.clone()).bind(delivery.name).bind(delivery.phone).bind(delivery.zip)
        .bind(delivery.city).bind(delivery.address).bind(delivery.region).bind(delivery.email)
        .execute(&mut *tx)
        .await;

    if result.is_err() {
        tx.rollback().await.expect("Cannot rollback");
        return Err(( StatusCode::INTERNAL_SERVER_ERROR, result.err().unwrap().to_string() ));
    }

    let payment = payload.payment.expect("Incorrect Payment Given");

    let mut payment_dt : Option<DateTime<chrono::Utc>> = None;
    if payment.payment_dt.is_some() {
        payment_dt = DateTime::from_timestamp(payment.payment_dt.unwrap(), 0);
    }

    let result : Result<_, _> = sqlx::query("insert into payment (order_uid, \"transaction\", request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) returning order_uid")
        .bind(order_uid.clone()).bind(payment.transaction).bind(payment.request_id).bind(payment.currency)
        .bind(payment.provider).bind(payment.amount)
        .bind(payment_dt)
        .bind(payment.bank)
        .bind(payment.delivery_cost).bind(payment.goods_total).bind(payment.custom_fee)
        .execute(&mut *tx)
        .await;

    if result.is_err() {
        tx.rollback().await.expect("Cannot rollback");
        return Err(( StatusCode::INTERNAL_SERVER_ERROR, result.err().unwrap().to_string() ));
    }

    let items = payload.items.expect("Incorrect Items Given");

    for item in items {
        let result : Result<_, _> = sqlx::query("insert into items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) returning order_uid")
            .bind(order_uid.clone()).bind(item.chrt_id).bind(item.track_number).bind(item.price)
            .bind(item.rid).bind(item.name).bind(item.sale).bind(item.size).bind(item.total_price)
            .bind(item.nm_id).bind(item.brand).bind(item.status)
            .execute(&mut *tx)
            .await;

        if result.is_err() {
            tx.rollback().await.expect("Cannot rollback");
            return Err(( StatusCode::INTERNAL_SERVER_ERROR, result.err().unwrap().to_string() ));
        }
    }

    let result : Result<_, _> = sqlx::query("insert into orders (order_uid, track_number, entry, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, oof_shard) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning order_uid")
        .bind(order_uid.clone()).bind(payload.track_number).bind(payload.entry).bind(payload.locale)
        .bind(payload.internal_signature).bind(payload.customer_id)
        .bind(payload.delivery_service)
        .bind(payload.shardkey)
        .bind(payload.sm_id).bind(payload.oof_shard)
        .execute(&mut *tx)
        .await;

    if result.is_err() {
        tx.rollback().await.expect("Cannot rollback");
        return Err(( StatusCode::INTERNAL_SERVER_ERROR, result.err().unwrap().to_string() ));
    }

    tx.commit().await.expect("Cannot commit transaction, panic");
    Ok("OK".to_string())

}