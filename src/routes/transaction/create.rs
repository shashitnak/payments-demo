use actix_web::{post, web};
use serde::{Deserialize, Serialize};
use sqlx::types::{chrono, BigDecimal, Uuid};

use crate::db::{self, account, transaction, TransactionType};
use crate::routes::{Result, UserExtractor};
use crate::AppData;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    from_account: i64,
    to_account: i64,
    #[serde(deserialize_with = "crate::utils::deserialize_from_str")]
    amount: BigDecimal,
    #[serde(skip)]
    #[serde(default = "crate::utils::now")]
    transaction_date: chrono::NaiveDateTime,
    description: String,
}

#[derive(Serialize)]
struct Response {
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    credit: Uuid,
    #[serde(serialize_with = "crate::utils::serialize_uuid")]
    debit: Uuid,
}

#[post("/createTransaction")]
pub async fn create_transaction(
    user_extractor: UserExtractor,
    app_data: web::Data<AppData>,
    req: web::Json<Request>,
) -> Result<web::Json<Response>> {
    let req = req.into_inner();
    let res = app_data
        .db_conn
        .with_transaction(move |db_conn| {
            let req = req.clone();
            let current_user = user_extractor.user.clone();
            async move {
                // Get account of sender
                let select_account = account::Select {
                    account_number: req.from_account,
                };
                let from_account = db_conn.run_query(select_account).await?;

                // Check if the account belongs to the current user
                if from_account.user_id != current_user.id {
                    Err(db::Error::UnauthorizedAccountAccess {
                        account_number: from_account.account_number,
                        user_id: current_user.id,
                    })?
                }

                // Check if sender has enough balance
                // for the transaction
                if from_account.balance < req.amount {
                    Err(db::Error::InsufficientBalance {
                        account_number: from_account.account_number,
                    })?
                }

                // Get account of receiver
                let select_account = account::Select {
                    account_number: req.to_account,
                };
                let to_account = db_conn.run_query(select_account).await?;

                // Insert debit transaction
                let debit_query = transaction::Insert {
                    account_id: from_account.id,
                    amount: &req.amount,
                    transaction_type: TransactionType::Debit,
                    transaction_date: req.transaction_date,
                    description: &req.description,
                };
                let debit = db_conn.run_query(debit_query).await?;

                // Update balance of sender
                let new_balance = from_account.balance - &req.amount;
                let update_account = account::Update {
                    id: from_account.id,
                    balance: &new_balance,
                };
                db_conn.run_query(update_account).await?;

                // Insert credit transaction
                let credit_query = transaction::Insert {
                    account_id: to_account.id,
                    amount: &req.amount,
                    transaction_type: TransactionType::Credit,
                    transaction_date: req.transaction_date,
                    description: &req.description,
                };
                let credit = db_conn.run_query(credit_query).await?;

                // Update balance of receiver
                let new_balance = to_account.balance + &req.amount;
                let update_account = account::Update {
                    id: from_account.id,
                    balance: &new_balance,
                };
                db_conn.run_query(update_account).await?;

                Ok(Response { credit, debit })
            }
        })
        .await?;

    Ok(web::Json(res))
}