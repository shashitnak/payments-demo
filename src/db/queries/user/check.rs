use crate::db::Error::UserNotFound;
use crate::db::Query;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub struct Check<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> Query<PgPool> for Check<'a> {
    type Output = Uuid;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
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
