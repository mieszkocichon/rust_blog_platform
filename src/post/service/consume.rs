use crate::database::PooledConnection;
use crate::post::model::Post;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use diesel::prelude::*;

pub(crate) fn consume_all_posts(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(posts
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Post>(conn)?)
}