use crate::db::{Account, Query};
use sqlx::types::BigDecimal;
use sqlx::PgPool;

pub struct Credit<'a> {
    pub account_number: i64,
    pub amount: &'a BigDecimal,
}

impl<'a> Query<PgPool> for Credit<'a> {
    type Output = Account;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
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
