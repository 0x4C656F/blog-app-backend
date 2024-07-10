use juniper::{ graphql_object, FieldResult };

use crate::graphql::Context;
use crate::services::blogs_service::*;
use crate::services::users_service::*;

pub struct ProtectedQuery;
pub struct ProtectedMutation;

#[graphql_object(context = Context)]
impl ProtectedQuery {
    async fn blogs(context: &Context) -> FieldResult<Vec<Blog>> {
        BlogsService::blogs(context).await
    }

    async fn blogs_by_user_id(user_id: i32, context: &Context) -> FieldResult<Vec<Blog>> {
        BlogsService::blogs_by_user_id(user_id, context).await
    }

    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        UsersService::find_all(context).await
    }

    async fn user(id: i32, context: &Context) -> FieldResult<User> {
        UsersService::find(id, context).await
    }

    async fn blog(id: i32, context: &Context) -> FieldResult<Blog> {
        BlogsService::blog(id, context).await
    }
}

#[juniper::graphql_object(context = Context)]
impl ProtectedMutation {
    async fn create_user(create_user_dto: CreateUserDto, context: &Context) -> FieldResult<User> {
        UsersService::create_user(create_user_dto, context).await
    }

    async fn create_blog(create_blog_dto: CreateBlogDto, context: &Context) -> FieldResult<Blog> {
        BlogsService::create_blog(create_blog_dto, context).await
    }

    async fn publish_blog(blog_id: i32, context: &Context) -> FieldResult<bool> {
        BlogsService::publish_blog(blog_id, context).await
    }
}
