
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;

use crate::model::Account;
use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn create_pool() -> Pool<sqlx::Postgres>{
    // Adjust the database URL as per your PostgreSQL configuration
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:password@localhost/database")
        .await
        .expect("Failed to create pool.");

    pool
}

pub async fn get_accounts(pool: &PgPool) -> Result<Vec<Account>, Error>{
    let accounts = sqlx::query_as::<_,Account>("SELECT * FROM Account")
        .fetch_all(pool)
        .await?;
    Ok(accounts)
}