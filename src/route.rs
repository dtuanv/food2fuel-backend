
use crate::handler::{create_item, get_accounts, get_item_by_id, get_items};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/accounts").route(web::get().to(get_accounts)))
    .service(web::resource("/items").route(web::get().to(get_items)))
     .service(web::resource("/item/{id}").route(web::get().to(get_item_by_id)))
        .service(web::resource("/save").route(web::post().to(create_item)));

}