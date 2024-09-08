use chrono::NaiveDateTime;

#[derive(
    serde_derive::Deserialize, serde_derive::Serialize, sqlx::FromRow, sqlx::Type, Hash, PartialEq, Eq, Debug, Clone
)]
pub struct Delivery {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub region: Option<String>,
    pub email: Option<String>,
}

#[derive(
    serde_derive::Deserialize, serde_derive::Serialize, sqlx::FromRow, sqlx::Type, Hash, PartialEq, Eq, Debug, Clone
)]
pub struct Payment {
    pub transaction: Option<String>,
    pub request_id: Option<String>,
    pub currency: Option<String>,
    pub provider: Option<String>,
    pub amount: Option<i32>,
    pub payment_dt: Option<NaiveDateTime>,
    pub bank: Option<String>,
    pub delivery_cost: Option<i32>,
    pub goods_total: Option<i32>,
    pub custom_fee: Option<i32>,
}

#[derive(
    serde_derive::Deserialize, serde_derive::Serialize, sqlx::FromRow, sqlx::Type, Hash, PartialEq, Eq, Debug, Clone
)]
pub struct Item {
    pub chrt_id: Option<i32>,
    pub track_number: Option<String>,
    pub price: Option<i32>,
    pub rid: Option<String>,
    pub name: Option<String>,
    pub sale: Option<i32>,
    pub size: Option<String>,
    pub total_price: Option<i32>,
    pub nm_id: Option<i32>,
    pub brand: Option<String>,
    pub status: Option<i32>,
}

#[derive(
    serde_derive::Deserialize, serde_derive::Serialize, sqlx::FromRow, Hash, PartialEq, Eq, Debug, Clone
)]
pub struct Order {
    pub order_uid: String,
    pub track_number: Option<String>,
    pub entry: Option<String>,
    pub delivery: Option<Delivery>,
    pub payment: Option<Payment>,
    pub items: Option<Vec<Item>>,
    pub locale: Option<String>,
    pub internal_signature: Option<String>,
    pub customer_id: Option<String>,
    pub delivery_service: Option<String>,
    pub shardkey: Option<String>,
    pub sm_id: Option<i32>,
    pub oof_shard: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}