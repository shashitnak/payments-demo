use crate::db::{self, Query, TransactionType};
use sqlx::types::{chrono, BigDecimal, Uuid};

pub struct Insert<'a> {
    pub account_id: Uuid,
    pub amount: &'a BigDecimal,
    pub transaction_type: TransactionType,
    pub transaction_date: chrono::NaiveDateTime,
    pub description: &'a str,
}

impl<'a> Query for Insert<'a> {
    type Output = Uuid;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> db::Result<Self::Output> {
        let record = sqlx::query!(
            r#"INSERT INTO transactions (
                    account_id,
                    amount,
                    transaction_type,
                    transaction_date,
                    description
                )
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id"#,
            self.account_id,
            self.amount,
            self.transaction_type as TransactionType,
            self.transaction_date,
            self.description
        )
        .fetch_one(conn)
        .await?;

        Ok(record.id)
    }
}
