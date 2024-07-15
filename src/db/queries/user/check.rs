use crate::db::Error::UserNotFound;
use crate::db::Query;
use sqlx::types::Uuid;

pub struct Check<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> Query for Check<'a> {
    type Output = Uuid;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> crate::db::Result<Self::Output> {
        let record = sqlx::query!(
            r#"SELECT id
                FROM users
                WHERE username = $1 AND password = $2
            "#,
            self.username,
            self.password
        )
        .fetch_one(conn)
        .await
        .map_err(|_| UserNotFound {
            username: Some(self.username.into()),
            id: None,
            email: None,
        })?;

        Ok(record.id)
    }
}
