use crate::db::{account, Account};
use crate::AppData;
use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    account_number: i64,
}

#[post("/getAccountDetail")]
pub async fn get_account_detail(
    req: web::Json<Request>,
    app_data: web::Data<AppData>,
) -> crate::routes::Result<web::Json<Account>> {
    let query = account::Select {
        account_number: req.account_number,
    };
    let account = app_data.db_conn.run_query(query).await?;
    Ok(web::Json(account))
}
