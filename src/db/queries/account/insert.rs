use crate::db;
use crate::db::{Account, Query};
use sqlx::types::BigDecimal;
use uuid::Uuid;

pub struct Insert<'a> {
    pub user_id: Uuid,
    pub balance: &'a BigDecimal,
}

impl<'a> Query for Insert<'a> {
    type Output = Account;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> db::Result<Self::Output> {
        let account = sqlx::query_as!(
            Account,
            r#"INSERT INTO accounts (
                    user_id,
                    balance
                )
                VALUES ($1, $2)
                RETURNING *"#,
            self.user_id,
            self.balance
        )
        .fetch_one(conn)
        .await?;

        Ok(account)
    }
}
