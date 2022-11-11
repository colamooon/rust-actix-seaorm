use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use log::info;


use super::post_handler;

async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    info!("]-----] validator._credentials [-----[ {:?}", _credentials);
    info!("]-----] validator.req [-----[ {:?}", req);
    Ok(req)
}

// this function could be located in a different module
pub fn post_route(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::basic(validator);
    cfg.service(
        web::scope("/api/v1/post").service(
            web::resource("")
                .route(web::get().to(post_handler::get_posts))
                .route(web::post().to(post_handler::add_post))
                .wrap(auth),
        ),
    );
}
