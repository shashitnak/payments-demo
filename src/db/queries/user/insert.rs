use crate::db::Error::UserAlreadyExists;
use crate::db::{self, Query};
use sqlx::types::Uuid;

pub struct Insert<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> Query for Insert<'a> {
    type Output = Uuid;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> db::Result<Self::Output> {
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
