use crate::db::{Query, Transaction, TransactionType};
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

pub struct SelectByAccount {
    pub account_id: Uuid,
}

impl Query<PgPool> for SelectByAccount {
    type Output = Vec<Transaction>;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Transaction,
            r#"SELECT
                    transaction_type AS "transaction_type!: TransactionType",
                    id,
                    account_id,
                    amount,
                    transaction_date,
                    description,
                    created_at,
                    updated_at
                FROM transactions
                WHERE account_id = $1
            "#,
            self.account_id
        )
        .fetch_all(conn)
        .await?)
    }
}

pub struct SelectByAccountWithStartTime {
    pub account_id: Uuid,
    pub start: chrono::NaiveDateTime,
}

impl Query<PgPool> for SelectByAccountWithStartTime {
    type Output = Vec<Transaction>;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Transaction,
            r#"SELECT
                    transaction_type AS "transaction_type!: TransactionType",
                    id,
                    account_id,
                    amount,
                    transaction_date,
                    description,
                    created_at,
                    updated_at
                FROM transactions
                WHERE account_id = $1 AND transaction_date >= $2
            "#,
            self.account_id,
            self.start
        )
        .fetch_all(conn)
        .await?)
    }
}

pub struct SelectByAccountWithEndTime {
    pub account_id: Uuid,
    pub end: chrono::NaiveDateTime,
}

impl Query<PgPool> for SelectByAccountWithEndTime {
    type Output = Vec<Transaction>;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Transaction,
            r#"SELECT
                    transaction_type AS "transaction_type!: TransactionType",
                    id,
                    account_id,
                    amount,
                    transaction_date,
                    description,
                    created_at,
                    updated_at
                FROM transactions
                WHERE account_id = $1 AND transaction_date <= $2
            "#,
            self.account_id,
            self.end
        )
        .fetch_all(conn)
        .await?)
    }
}

pub struct SelectByAccountWithStartAndEnd {
    pub account_id: Uuid,
    pub start: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
}

impl Query<PgPool> for SelectByAccountWithStartAndEnd {
    type Output = Vec<Transaction>;

    async fn execute(&self, conn: &PgPool) -> crate::db::Result<Self::Output> {
        Ok(sqlx::query_as!(
            Transaction,
            r#"SELECT
                    transaction_type AS "transaction_type!: TransactionType",
                    id,
                    account_id,
                    amount,
                    transaction_date,
                    description,
                    created_at,
                    updated_at
                FROM transactions
                WHERE account_id = $1 AND transaction_date >= $2 AND transaction_date <= $3
            "#,
            self.account_id,
            self.start,
            self.end
        )
        .fetch_all(conn)
        .await?)
    }
}
