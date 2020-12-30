use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::post::model::{InsertablePost, SlimPost, Post, PostData};
use actix_web::web;
use diesel::prelude::*;
use crate::user::model::{SlimUser};

pub fn publish(slim_user: SlimUser, post_data: PostData, pool: web::Data<Pool>) -> ServiceResult<SlimPost> {
    let conn = &db_connection(&pool)?;
    create_post(slim_user, post_data, conn)
}

pub fn create_post(slim_user: SlimUser, post_data: PostData, conn: &PgConnection) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::posts;

    let mut post: InsertablePost = post_data.into();
    post.owner_id = slim_user.email;

    let inserted_post: Post = diesel::insert_into(posts).values(&post).get_result(conn)?;
    Ok(inserted_post.into())
}