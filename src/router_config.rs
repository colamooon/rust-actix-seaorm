use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use log::info;

use crate::post;
async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    info!("]-----] validator._credentials [-----[ {:?}", _credentials);
    info!("]-----] validator.req [-----[ {:?}", req);
    Ok(req)
}
pub fn config_router(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::basic(validator);
    cfg.service(
        web::scope("/api/v1/post")
            .configure(post::post_router::post_route)
            .wrap(auth),
    );
}
