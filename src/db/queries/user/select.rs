use crate::db::Error::UserNotFound;
use crate::db::{Query, User};
use sqlx::types::Uuid;
use sqlx::PgPool;

pub struct Select {
    pub id: Uuid,
}

impl Query<PgPool> for Select {
    type Output = User;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
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
