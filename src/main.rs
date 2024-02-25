use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;

mod model;
mod db;
mod route;
mod handler;
use model:: Account;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Start Actix web server
    let pool = db::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(route::configure_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}
