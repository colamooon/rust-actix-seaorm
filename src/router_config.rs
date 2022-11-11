use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use log::info;

use crate::post;

pub fn config_router(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg.configure(post::post_router::post_route);
}
