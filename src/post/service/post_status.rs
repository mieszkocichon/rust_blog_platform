use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::post::model::{SlimPost, Post, PostDataStatusRq};
use actix_web::web;
use diesel::prelude::*;

pub fn disable(put_data: PostDataStatusRq, pool: web::Data<Pool>) -> ServiceResult<SlimPost> {
    let conn = &db_connection(&pool)?;
    disable_post(put_data, conn)
}

pub fn disable_post(put_data: PostDataStatusRq, conn: &PgConnection) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::*;

    let id = put_data.id;
    let post: Post = posts.find(id).get_result::<Post>(conn).expect(&format!("Unable to find post"));

    change_post_status(conn, post, false)
}

pub fn enable(put_data: PostDataStatusRq, pool: web::Data<Pool>) -> ServiceResult<SlimPost> {
    let conn = &db_connection(&pool)?;
    enable_post(put_data, conn)
}

pub fn enable_post(put_data: PostDataStatusRq, conn: &PgConnection) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::*;

    let id = put_data.id;
    let post: Post = posts.find(id).get_result::<Post>(conn).expect(&format!("Unable to find post"));

    change_post_status(conn, post, true)
}

pub fn change_post_status(conn: &PgConnection, post: Post, status: bool) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::*;

    let post_update = diesel::update(&post)
        .set((
            published.eq(status),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Post>(conn)?;
    
    Ok(post_update.into())
}