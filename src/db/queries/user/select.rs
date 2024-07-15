use crate::db::Error::UserNotFound;
use crate::db::{Query, User};
use sqlx::types::Uuid;

pub struct Select {
    pub id: Uuid,
}

impl Query for Select {
    type Output = User;

    async fn execute<'b>(
        &self,
        conn: impl sqlx::Executor<'b, Database = sqlx::Postgres>,
    ) -> crate::db::Result<Self::Output> {
        sqlx::query_as!(
            User,
            r#"SELECT *
                FROM users
                WHERE id = $1
            "#,
            self.id
        )
        .fetch_one(conn)
        .await
        .map_err(|_| UserNotFound {
            id: Some(self.id),
            username: None,
            email: None,
        })
    }
}
