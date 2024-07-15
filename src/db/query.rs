use crate::db;

#[allow(async_fn_in_trait)]
pub trait Query {
    type Output;
    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> db::Result<Self::Output>;
}
