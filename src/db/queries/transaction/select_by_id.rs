use crate::db::{Query, Transaction, TransactionType};
use sqlx::PgPool;
use uuid::Uuid;

pub struct SelectById {
    pub id: Uuid,
}

impl Query<PgPool> for SelectById {
    type Output = Transaction;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Transaction,
            r#"SELECT
                    transaction_type AS "transaction_type: TransactionType",
                    id,
                    account_id,
                    amount,
                    transaction_date,
                    description,
                    created_at,
                    updated_at
                FROM transactions
                WHERE id = $1
            "#,
            self.id
        )
        .fetch_one(conn)
        .await?)
    }
}
