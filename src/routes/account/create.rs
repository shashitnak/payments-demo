use crate::db::{account, Account};
use crate::routes::user_extractor::UserExtractor;
use crate::AppData;
use actix_web::{post, web};
use serde::Deserialize;
use sqlx::types::BigDecimal;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    #[serde(default = "BigDecimal::default", deserialize_with = "crate::utils::deserialize_from_str")]
    opening_balance: BigDecimal,
}

#[post("/createAccount")]
pub async fn create_account(
    user_extractor: UserExtractor,
    app_data: web::Data<AppData>,
    req: web::Json<Request>,
) -> crate::routes::Result<web::Json<Account>> {
    let query = account::Insert {
        user_id: user_extractor.user.id,
        balance: &req.opening_balance,
    };
    let account = app_data.db_conn.run_query(query).await?;

    Ok(web::Json(account))
}
