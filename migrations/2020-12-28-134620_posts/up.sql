-- Your SQL goes here
CREATE TABLE posts (
    post_id serial PRIMARY KEY,
    title varchar NOT NULL,
    raw_content text NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at timestamp without time zone NOT NULL,
    post_type integer NOT NULL,
    published boolean DEFAULT false NOT NULL
);
