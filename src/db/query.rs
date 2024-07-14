use crate::db;

#[allow(async_fn_in_trait)]
pub trait Query<DBConn> {
    type Output;
    async fn execute(&self, conn: &DBConn) -> db::Result<Self::Output>;
}
