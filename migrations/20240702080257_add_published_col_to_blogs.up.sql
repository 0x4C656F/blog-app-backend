-- Add up migration script here
alter table blogs
add column published boolean default false;