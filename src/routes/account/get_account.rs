use crate::db::{account, Account};
use crate::routes::UserExtractor;
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
    user_extractor: UserExtractor,
    req: web::Json<Request>,
    app_data: web::Data<AppData>,
) -> crate::routes::Result<web::Json<Account>> {
    let current_user = user_extractor.user;
    let query = account::Select {
        account_number: req.account_number,
    };
    let account = app_data.db_conn.run_query(query).await?;

    if account.user_id != current_user.id {
        Err(crate::db::Error::UnauthorizedAccountAccess {
            account_number: req.account_number,
            user_id: current_user.id,
        })?
    }

    Ok(web::Json(account))
}
