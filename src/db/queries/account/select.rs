use crate::db::{Account, Query};
use sqlx::PgPool;

pub struct Select {
    pub account_number: i64,
}

impl Query<PgPool> for Select {
    type Output = Account;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Account,
            r#"SELECT *
                FROM accounts
                WHERE account_number = $1
            "#,
            self.account_number
        )
        .fetch_one(conn)
        .await?)
    }
}
