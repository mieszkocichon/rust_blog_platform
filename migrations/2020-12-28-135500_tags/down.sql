-- This file should undo anything in `up.sql`
alter table posts
drop column tags;
drop index if exists idx_tag;