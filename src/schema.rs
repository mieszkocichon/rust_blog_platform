table! {
    posts (post_id) {
        post_id -> Int4,
        title -> Varchar,
        raw_content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        post_type -> Int4,
        published -> Bool,
        tags -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_uuid -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        role -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
