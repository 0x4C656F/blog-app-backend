use juniper::{ graphql_object, FieldResult };

use crate::graphql::Context;
use crate::services::blogs_service::*;
use crate::services::users_service::*;
pub fn get_user_id_or_throw(context: &crate::graphql::Context) -> Result<i32, juniper::FieldError> {
    context.user_id.ok_or_else(|| juniper::FieldError::new("Unauthorized", juniper::Value::Null))
}

pub struct ProtectedQuery;
pub struct ProtectedMutation;

#[graphql_object(context = Context)]
impl ProtectedQuery {
    async fn blogs(context: &Context) -> FieldResult<Vec<Blog>> {
        println!("[{:?}] BlogsService /blogs", chrono::Utc::now());
        let id = get_user_id_or_throw(context)?;
        BlogsService::blogs(context).await
    }

    async fn blogs_by_user_id(user_id: i32, context: &Context) -> FieldResult<Vec<Blog>> {
        println!(
            "[{:?}] BlogsService /blogs_by_user_id with user_id: {:?}",
            chrono::Utc::now(),
            &user_id
        );
        let id = get_user_id_or_throw(context)?;

        BlogsService::blogs_by_user_id(user_id, context).await
    }

    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        println!("[{:?}] UsersService /users", chrono::Utc::now());
        let id = get_user_id_or_throw(context)?;

        UsersService::find_all(context).await
    }

    async fn user(id: i32, context: &Context) -> FieldResult<User> {
        println!("[{:?}] UsersService /user with id: {:?}", chrono::Utc::now(), &id);
        let id = get_user_id_or_throw(context)?;

        UsersService::find(id, context).await
    }

    async fn me(context: &Context) -> FieldResult<User> {
        println!("[{:?}] UsersService /me", chrono::Utc::now());
        let id = get_user_id_or_throw(context)?;

        UsersService::find(id, context).await
    }

    async fn blog(id: i32, context: &Context) -> FieldResult<Blog> {
        println!("[{:?}] BlogsService /blog with id: {:?}", chrono::Utc::now(), &id);
        let id = get_user_id_or_throw(context)?;

        BlogsService::blog(id, context).await
    }

    fn check(context: &Context) -> FieldResult<bool> {
        println!("[{:?}] ProtectedQuery /check", chrono::Utc::now());
        let id = get_user_id_or_throw(context)?;

        Ok(true)
    }
}

#[juniper::graphql_object(context = Context)]
impl ProtectedMutation {
    async fn create_user(create_user_dto: CreateUserDto, context: &Context) -> FieldResult<User> {
        println!(
            "[{:?}] UsersService /create_user with dto: {:?}",
            chrono::Utc::now(),
            &create_user_dto
        );
        let id = get_user_id_or_throw(context)?;

        UsersService::create_user(create_user_dto, context).await
    }

    async fn create_blog(create_blog_dto: CreateBlogDto, context: &Context) -> FieldResult<Blog> {
        println!(
            "[{:?}] BlogsService /create_blog with dto: {:?}",
            chrono::Utc::now(),
            &create_blog_dto
        );
        let id = get_user_id_or_throw(context)?;

        BlogsService::create_blog(create_blog_dto, context).await
    }

    async fn publish_blog(blog_id: i32, context: &Context) -> FieldResult<bool> {
        println!(
            "[{:?}] BlogsService /publish_blog with blog_id: {:?}",
            chrono::Utc::now(),
            &blog_id
        );
        let id = get_user_id_or_throw(context)?;

        BlogsService::publish_blog(blog_id, context).await
    }
}
