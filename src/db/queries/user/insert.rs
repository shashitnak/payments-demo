use crate::db::Error::UserAlreadyExists;
use crate::db::{self, Query};
use sqlx::types::Uuid;
use sqlx::PgPool;

pub struct Insert<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> Query<PgPool> for Insert<'a> {
    type Output = Uuid;

    async fn execute(&self, conn: &PgPool) -> db::Result<Self::Output> {
        let record = sqlx::query!(
            r#"INSERT INTO users (name, email, username, password)
                VALUES ($1, $2, $3, $4)
                RETURNING id"#,
            self.name,
            self.email,
            self.username,
            self.password
        )
        .fetch_one(conn)
        .await
        .map_err(|_| UserAlreadyExists)?;

        Ok(record.id)
    }
}
