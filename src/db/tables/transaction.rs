use serde::{Deserialize, Serialize};
use sqlx::types::{chrono, BigDecimal, Uuid};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "transaction_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    pub id: Uuid,
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    pub account_id: Uuid,
    #[serde(serialize_with = "crate::utils::serialize_display")]
    pub amount: BigDecimal,
    pub transaction_type: TransactionType,
    #[serde(serialize_with = "crate::utils::serialize_time")]
    pub transaction_date: chrono::NaiveDateTime,
    pub description: String,
    #[serde(skip)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub updated_at: chrono::NaiveDateTime,
}
