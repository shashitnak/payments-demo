use crate::db::{Query, User};
use sqlx::types::Uuid;
use sqlx::{FromRow, PgPool};
use std::fmt::Write;

pub struct Update<'a> {
    pub id: Uuid,
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

impl<'a> Query<PgPool> for Update<'a> {
    type Output = User;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        let mut query = "UPDATE users SET ".to_string();

        let args = [
            self.name.map(|val| ("name", val)),
            self.email.map(|val| ("email", val)),
            self.username.map(|val| ("username", val)),
            self.password.map(|val| ("password", val)),
        ]
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, (name, val))| {
            if index > 0 {
                write!(&mut query, ", ")?;
            }
            write!(&mut query, "{name} = ${}", index + 1)?;
            Ok(val)
        })
        .collect::<crate::db::Result<Vec<_>>>()?;

        write!(&mut query, " WHERE id = ${} RETURNING *", args.len() + 1)?;

        let mut query = sqlx::query(&query);

        for arg in args.into_iter() {
            query = query.bind(arg);
        }

        query = query.bind(self.id);

        let row = query.fetch_one(conn).await?;

        Ok(User::from_row(&row)?)
    }
}
