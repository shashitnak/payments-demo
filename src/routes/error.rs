use actix_web::ResponseError;
use derive_more::From;
use serde::Serialize;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, From)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    JWTCreationFailed,
    JWTValidationFailed,
    NoLoggedInUser,
    InsufficientBalance,
    #[from]
    DB(crate::db::Error),
    #[from]
    #[serde(serialize_with = "crate::utils::serialize_debug")]
    Sqlx(sqlx::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl ResponseError for Error {}
