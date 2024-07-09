mod users_service;
pub use users_service::*;

#[derive(sqlx::FromRow, juniper::GraphQLObject, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(juniper::GraphQLInputObject,Clone)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
}
