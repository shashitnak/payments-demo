use actix_web::ResponseError;
use derive_more::From;
use serde::Serialize;
use sqlx::types::Uuid;
use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    UserAlreadyExists,
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    UserNotFound {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<Uuid>,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
    },
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    UnauthorizedAccountAccess {
        account_number: i64,
        user_id: Uuid,
    },
    AccountDoesNotExist {
        account_number: i64,
    },
    InsufficientBalance {
        account_number: i64,
    },
    #[from]
    #[serde(serialize_with = "crate::utils::serialize_debug")]
    SqlxError(sqlx::Error),
    #[from]
    #[serde(serialize_with = "crate::utils::serialize_debug")]
    Migration(sqlx::migrate::MigrateError),
    #[from]
    #[serde(serialize_with = "crate::utils::serialize_debug")]
    Fmt(std::fmt::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl ResponseError for Error {}
