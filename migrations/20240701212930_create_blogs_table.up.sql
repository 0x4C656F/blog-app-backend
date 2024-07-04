-- Up Migration
create table if not exists blogs (
    id         serial primary key,
    title      varchar(255) not null,
    content    text not null,
    user_id    int not null,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp,
    constraint fk_blogs_user foreign key (user_id) references users (id) on delete cascade
);

create index if not exists idx_blogs_user_id on blogs (user_id);
