use crate::db::Query;
use crate::db::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct DBConn {
    pub conn: PgPool,
}

impl DBConn {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = PgPool::connect(url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { conn: pool })
    }

    pub async fn run_query<Q: Query>(&self, query: Q) -> Result<Q::Output> {
        query.execute(&self.conn).await
    }
}
