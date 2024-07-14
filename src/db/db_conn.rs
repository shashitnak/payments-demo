use crate::db::Query;
use crate::db::Result;
use sqlx::PgPool;
use std::future::Future;

#[derive(Clone)]
pub struct DBConn {
    conn: PgPool,
}

impl DBConn {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = PgPool::connect(url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { conn: pool })
    }

    pub async fn run_query<Q: Query<PgPool>>(&self, query: Q) -> Result<Q::Output> {
        query.execute(&self.conn).await
    }

    pub async fn with_transaction<F, U, Fut>(&self, mut f: F) -> Result<U>
    where
        F: FnMut(Self) -> Fut,
        Fut: Future<Output = Result<U>>,
    {
        let transaction = self.conn.begin().await?;
        let result = f(self.clone()).await?;
        transaction.commit().await?;
        Ok(result)
    }
}
