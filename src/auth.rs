use std::future::Future;

use actix_web::{dev::ServiceRequest, http::Error};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use log::info;

pub async fn validator(
    req: ServiceRequest,
    _credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    info!("]-----] validator._credentials [-----[ {:?}", _credentials);
    info!("]-----] validator.req [-----[ {:?}", req);
    Ok(req)
}
