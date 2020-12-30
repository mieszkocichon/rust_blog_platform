use crate::schema::*;
use chrono::*;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub raw_content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub post_type: i32,
    pub published: bool,
    pub tags: String,
    pub owner_id: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub title: String,
    pub raw_content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub post_type: i32,
    pub published: bool,
    pub tags: String,
    pub owner_id: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct PostData {
    pub title: String,
    pub raw_content: String,
    pub post_type: i32,
    pub tags: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimPost {
    pub title: String,
    pub raw_content: String,
    pub tags: String,
}

impl From<PostData> for InsertablePost {
    fn from(post_data: PostData) -> Self {
        let PostData {
            title,
            raw_content,
            post_type,
            tags,
        } = post_data;

        Self {
            title,
            raw_content,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            post_type,
            published: true,
            tags,
            owner_id: "".to_string(),
        }
    }
}
impl From<Post> for SlimPost {
    fn from(post: Post) -> Self {
        let Post {
            title,
            raw_content,
            tags,
            ..
        } = post;

        Self {
            title,
            raw_content,
            tags,
        }
    }
}