use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use log::info;


use super::post_handler;



// this function could be located in a different module
pub fn post_route(cfg: &mut web::ServiceConfig) {
    
    cfg.service(
        // web::scope("/api/v1/post").service(
            web::resource("")
                .route(web::get().to(post_handler::get_posts))
                .route(web::post().to(post_handler::add_post))
                ,
        // ),
    );
}
