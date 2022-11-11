use ::entity::{post, post::Entity as Post};
use chrono::Utc;
use sea_orm::*;

pub struct PostRepository;

impl PostRepository {
    pub async fn find_post_by_id(db: &DbConn, id: i64) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }

    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64, u64), DbErr> {
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;
        let total_count =paginator.num_items().await?; 
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages, total_count))
    }

    pub async fn create(db: &DbConn, new_post: post::ActiveModel) -> Result<post::Model, DbErr> {
        let _now = Utc::now();
        // new_post.created_at = Some(now);
        
        new_post.insert(db).await
    }

    // pub async fn update_post_by_id(
    //     db: &DbConn,
    //     id: i64,
    //     form_data: post::Model,
    // ) -> Result<post::Model, DbErr> {
    //     let post: post::ActiveModel = Post::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
    //         .map(Into::into)?;

    //     post::ActiveModel {
    //         id: post.id,
    //         title: Set(form_data.title.to_owned()),
    //         body: Set(form_data.body.to_owned()),
    //     }
    //     .update(db)
    //     .await
    // }

    pub async fn delete_post(db: &DbConn, id: i64) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }
}
