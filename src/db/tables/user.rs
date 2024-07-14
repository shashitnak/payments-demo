use serde::Serialize;
use sqlx::types::{chrono, Uuid};

#[derive(Clone, Debug, sqlx::FromRow, Serialize)]
pub struct User {
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub updated_at: chrono::NaiveDateTime,
}
