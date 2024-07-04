-- Add down migration script here
alter table blogs
drop column if exists published;