use serde::Serialize;
use sqlx::types::{chrono, BigDecimal, Uuid};

#[derive(Clone, Debug, sqlx::FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    pub id: Uuid,
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    pub user_id: Uuid,
    pub account_number: i64,
    #[serde(serialize_with = "crate::utils::serialize_display")]
    pub balance: BigDecimal,
    #[serde(skip)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub updated_at: chrono::NaiveDateTime,
}
