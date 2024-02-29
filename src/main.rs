use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{http, get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use actix_web::http::header::ContentEncoding;
use sqlx::postgres::PgPoolOptions;
use log::info;

mod model;
mod db;
mod route;
mod handler;
use model:: Account;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let pool = db::create_pool().await;
    // Start Actix web server
    info!("Starting server at 127.0.0.1:8080");
    HttpServer::new(move || {
        // Construct CORS middleware
        let cors = Arc::new(Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"]));


        App::new()

            .app_data(web::Data::new(cors.clone())) // Make CORS instance available to handlers
            .app_data(web::Data::new(pool.clone()))
            .configure(route::configure_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
