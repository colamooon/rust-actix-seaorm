use actix_web::{web, Error, HttpResponse};

use crate::models::{AppState, PageInfo};

use super::{post::NewPostReq, post_service};

pub async fn get_posts(
    data: web::Data<AppState>,
    page_info: web::Query<PageInfo>,
) -> Result<HttpResponse, Error> {
    print!("]-----] post_handler::get_posts [------[");
    let conn = &data.conn;
    let (posts, _num_pages) = post_service::find_all_post(conn, page_info.page, page_info.size)
        .await
        .expect("Cannot find posts in page");
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn add_post(
    data: web::Data<AppState>,
    new_post_req: web::Json<NewPostReq>,
) -> Result<HttpResponse, Error> {
    print!("]-----] post_handler::add_post [------[");
    let conn = &data.conn;
    let post = post_service::insert_new_post(conn, &new_post_req)
        .await
        .expect("could not insert post");

    Ok(HttpResponse::Ok().json(post))
}
