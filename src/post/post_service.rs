use super::post::NewPostReq;
use ::entity::{post, post::Entity as Post};
use log::info;
use migration::DbErr;

use sea_orm::*;

pub async fn find_all_post(
    conn: &DbConn,
    page: u64,
    size: u64,
) -> Result<(Vec<post::Model>, u64), DbErr> {
    info!("]-----] post_service::find_all_post [------[ {}", page);
    let paginator = Post::find()
        .order_by_asc(post::Column::Id)
        // .into_json()
        .paginate(conn, size);
    let num_pages = paginator.num_pages().await?;
    paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
}

pub async fn insert_new_post(
    conn: &DbConn,
    new_post_req: &NewPostReq,
) -> Result<post::Model, DbErr> {
    print!(
        "]-----] post_service::insert_new_post.new_post_req [------[ {:?}",
        new_post_req
    );
    let new_post = post::ActiveModel {
        title: Set(new_post_req.title.to_owned()),
        body: Set(new_post_req.body.to_owned()),
        ..Default::default()
    };

    new_post.insert(conn).await
}
