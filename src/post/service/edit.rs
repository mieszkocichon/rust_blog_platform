use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::post::model::{SlimPost, Post, PostDataEdit};
use diesel::prelude::*;
use crate::user::model::{SlimUser};
use actix_web::{web};

pub fn edit(slim_user: SlimUser, put_data: PostDataEdit, pool: web::Data<Pool>) -> ServiceResult<SlimPost> {
    let conn = &db_connection(&pool)?;
    edit_post(slim_user, put_data, conn)
}

pub fn edit_post(slim_user: SlimUser, put_data: PostDataEdit, conn: &PgConnection) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::*;

    let id = put_data.id;
    let post: Post = posts.find(id).get_result::<Post>(conn).expect(&format!("Unable to find post"));

    let post_updated = diesel::update(&post)
        .set((
            title.eq(put_data.title),
            raw_content.eq(put_data.raw_content),
            post_type.eq(put_data.post_type),
            tags.eq(put_data.tags),
            updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_result::<Post>(conn)?;

    let result = match slim_user.role.as_ref() {
        "admin" => {
            post_updated.into()
        },
        _ => {
            // !TODO: WARNING post.tags to post.owner
            if slim_user.email == post.tags {
                post_updated.into()
            }
            else {
                SlimPost {
                    title: "".to_string(),
                    raw_content: "".to_string(),
                    tags: "".to_string(),
                    id: 0,
                    published: false
                }
            }
        },
    };

    Ok(result)
}