-- Your SQL goes here
alter table posts
add column tags VARCHAR(255) DEFAULT '["programming"]' NOT NULL;
create index idx_tag on posts(tags);