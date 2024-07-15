use crate::db::{Account, Query};

pub struct Select {
    pub account_number: i64,
}

impl Query for Select {
    type Output = Account;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> crate::db::Result<Self::Output> {
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
