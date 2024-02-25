use crate::db;
use crate::model::Account;
use actix_web::{web, HttpResponse, Responder};

// Handler to get all accounts
pub async fn get_accounts(pool: web :: Data<sqlx::PgPool>) -> impl Responder {
    let result = db::get_accounts(pool.get_ref()).await;
    match result {
        Ok(accounts) => HttpResponse::Ok().json(accounts),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching accounts"),
    }
}