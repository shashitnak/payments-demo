use crate::db::{Account, Query};
use sqlx::types::BigDecimal;

pub struct Credit<'a> {
    pub account_number: i64,
    pub amount: &'a BigDecimal,
}

impl<'a> Query for Credit<'a> {
    type Output = Account;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Account,
            r#"UPDATE accounts
            SET balance = balance + $1
            WHERE account_number = $2
            RETURNING *
        "#,
            self.amount,
            self.account_number
        )
        .fetch_one(conn)
        .await?)
    }
}
