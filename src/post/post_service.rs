use ::entity::{post, post::Entity as Post};
use chrono::Utc;
use log::info;
use migration::DbErr;

use sea_orm::*;

use crate::{models::Pageable, post::post_repository::PostRepository};

use super::post_model::{NewPostReq, PostRes};

fn render(post: post::Model) -> PostRes {
    PostRes {
        id: post.id,
        active: post.active,
        created_at: post.created_at,
        created_id: post.created_id,
        updated_at: post.updated_at,
        updated_id: post.updated_id,
        title: post.title,
        content: post.content,
    }
}
pub async fn find_all_post(
    conn: &DbConn,
    page: u64,
    size: u64,
) -> Result<Pageable<PostRes>, DbErr> {
    info!("]-----] post_service::find_all_post [------[ {}", page);
    let (posts, num_pages, total_count) =
        PostRepository::find_posts_in_page(conn, page, size).await?;
    let mut post_res: Vec<PostRes> = Vec::new();

    for post_out in posts {
        post_res.push(self::render(post_out));
    }
    Ok(Pageable {
        content: post_res,
        is_last: false,
        page: page,
        size: size,
        num_pages: num_pages,
        total_count: total_count,
    })
}

pub async fn insert_new_post(conn: &DbConn, new_post_req: &NewPostReq) -> Result<PostRes, DbErr> {
    print!(
        "]-----] post_service::insert_new_post.new_post_req [------[ {:?}",
        new_post_req
    );
    let _now = Utc::now();
    let new_post = post::ActiveModel {
        title: Set(new_post_req.title.to_owned()),
        content: Set(new_post_req.content.to_owned()),
        created_at: Set(Some(_now)),
        updated_at: Set(Some(_now)),
        ..Default::default()
    };
    Ok(self::render(PostRepository::create(conn, new_post).await?))
}
