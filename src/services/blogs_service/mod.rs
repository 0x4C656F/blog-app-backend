
mod blogs_service;
pub use blogs_service::*;

#[derive(sqlx::FromRow, juniper::GraphQLObject, Debug)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(juniper::GraphQLInputObject)]
pub struct CreateBlogDto {
    pub title: String,
    pub content: String,
}
