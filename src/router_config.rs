use actix_web::web;

use crate::post;

pub fn config_router(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg.configure(post::post_router::post_route);
}
