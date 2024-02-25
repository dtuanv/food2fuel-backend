
use crate::handler::{get_accounts};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/accounts").route(web::get().to(get_accounts)));

}