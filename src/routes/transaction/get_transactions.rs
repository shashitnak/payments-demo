use crate::db::{account, transaction, Transaction};
use crate::routes::UserExtractor;
use crate::utils::str_to_time;
use crate::AppData;
use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    account_number: i64,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[post("/getTransactions")]
pub async fn get_transaction_list(
    user_extractor: UserExtractor,
    req: web::Json<Request>,
    app_data: web::Data<AppData>,
) -> crate::routes::Result<web::Json<Vec<Transaction>>> {
    let current_user = user_extractor.user;

    let account_number = req.account_number;
    let start = req.start_date.as_deref().and_then(str_to_time);
    let end = req.end_date.as_deref().and_then(str_to_time);

    let query = account::Select { account_number };
    let account = app_data.db_conn.run_query(query).await?;
    if account.user_id != current_user.id {
        return Err(crate::db::Error::UnauthorizedAccountAccess {
            account_number: account.account_number,
            user_id: current_user.id,
        })?;
    }

    let account_id = account.id;
    let transaction = match (start, end) {
        (Some(start), Some(end)) => {
            let query = transaction::SelectByAccountWithStartAndEnd {
                account_id,
                start,
                end,
            };
            app_data.db_conn.run_query(query).await
        }
        (Some(start), None) => {
            let query = transaction::SelectByAccountWithStartTime { account_id, start };
            app_data.db_conn.run_query(query).await
        }
        (None, Some(end)) => {
            let query = transaction::SelectByAccountWithEndTime { account_id, end };
            app_data.db_conn.run_query(query).await
        }
        (None, None) => {
            let query = transaction::SelectByAccount { account_id };

            app_data.db_conn.run_query(query).await
        }
    }?;

    Ok(web::Json(transaction))
}
