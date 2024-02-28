use crate::db;
use crate::model::{Account, Item};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json; // Import for JSON serialization/deserialization

// Handler to get all accounts
pub async fn get_accounts(pool: web :: Data<sqlx::PgPool>) -> impl Responder {
    let result = db::get_accounts(pool.get_ref()).await;
    match result {
        Ok(accounts) => HttpResponse::Ok().json(accounts),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching accounts"),
    }
}

pub async fn get_items(pool: web :: Data<sqlx::PgPool>) -> impl Responder {
    let result = db::get_items(pool.get_ref()).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching accounts"),
    }
}
pub async fn get_item_by_id(path: web::Path<(i64,)>,pool: web :: Data<sqlx::PgPool>) -> impl Responder {
    let id = (path.0);

    let result = db::get_item_by_id(pool.get_ref(), id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching accounts"),
    }
}

pub async fn create_item(pool: web :: Data<sqlx::PgPool> ,item: web::Json<Item>) -> impl Responder  {
    let item = item.into_inner(); // Extract the Item object from web::Json
    let result = db::insert_item(pool.get_ref(), item).await;
    match result {
        Ok(inserted_item) => HttpResponse::Created().json(inserted_item),
        Err(_) => HttpResponse::InternalServerError().body("Error inserting item into database"),
    }
}
