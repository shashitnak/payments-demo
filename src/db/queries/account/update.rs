use crate::db::Query;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct Update<'a> {
    pub id: Uuid,
    pub balance: &'a BigDecimal,
}

impl<'a> Query<PgPool> for Update<'a> {
    type Output = ();

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        sqlx::query!(
            r#"UPDATE accounts
            SET balance = $1
            WHERE id = $2
        "#,
            self.balance,
            self.id
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}
