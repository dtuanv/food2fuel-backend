
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use actix_web::{HttpResponse, Responder};
use crate::model::{Account, Item};
use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn create_pool() -> Pool<sqlx::Postgres>{
    // Adjust the database URL as per your PostgreSQL configuration
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:**@localhost/food2fuel")
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

pub async fn get_items(pool: &PgPool) -> Result<Vec<Item>, Error> {
    let items = sqlx::query_as::<_,Item>("SELECT * FROM Item")
        .fetch_all(pool).await?;
    Ok(items)
}

pub async fn get_item_by_id(pool: &PgPool, id: i64) -> Result<Vec<Item>, Error> {
    let items = sqlx::query_as::<_,Item>("SELECT * FROM Item WHERE id = $1")
        .bind(id)
        .fetch_all(pool).await?;
    Ok(items)
}

pub async fn insert_item(pool: &PgPool, item: Item) -> Result<Item, sqlx::Error> {
    let inserted_item = sqlx::query_as::<_, Item>(
        "INSERT INTO item (description, category, expirationdate) VALUES ($1, $2, $3) RETURNING id, description, category, expirationdate"
    )
        .bind(&item.description)
        .bind(&item.category)
        .bind(&item.expirationdate)
        .fetch_one(pool)
        .await?;

    Ok(inserted_item)
}