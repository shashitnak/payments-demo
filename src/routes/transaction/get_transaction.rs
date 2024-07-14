use crate::db::{transaction, Transaction};
use crate::AppData;
use actix_web::{post, web};
use serde::Deserialize;
use sqlx::types::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    #[serde(deserialize_with = "crate::utils::deserialize_uuid")]
    transaction_id: Uuid,
}

#[post("/getTransaction")]
pub async fn get_transaction_by_id(
    req: web::Json<Request>,
    app_data: web::Data<AppData>,
) -> crate::routes::Result<web::Json<Transaction>> {
    let transaction_id = req.transaction_id;
    let query = transaction::SelectById {
        id: dbg!(transaction_id),
    };

    let transaction = app_data.db_conn.run_query(query).await?;
    Ok(web::Json(transaction))
}
