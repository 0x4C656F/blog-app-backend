use blogs_service::BlogsService;
use blogs_service::IBlogsService;
use juniper::EmptySubscription;
use juniper::FieldResult;
use sqlx::PgPool;
use users_service::IUsersService;
use crate::services::users_service::*;
use crate::services::blogs_service::*;

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        UsersService::get_all(context).await
    }

    async fn user(id: i32, context: &Context) -> FieldResult<User> {
        UsersService::get(id, context).await
    }

    async fn blogs(context: &Context) -> FieldResult<Vec<Blog>> {
        BlogsService::blogs(context).await
    }

    async fn blogs_by_user_id(user_id: i32, context: &Context) -> FieldResult<Vec<Blog>> {
        BlogsService::blogs_by_user_id(user_id, context).await
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_user(create_user_dto: CreateUserDto, context: &Context) -> FieldResult<User> {
        UsersService::create(create_user_dto, context).await
    }

    async fn create_blog(create_blog_dto: CreateBlogDto, context: &Context) -> FieldResult<Blog> {
        BlogsService::create_blog(create_blog_dto, context).await
    }

    async fn publish_blog(blog_id: i32, context: &Context) -> FieldResult<bool> {
        BlogsService::publish_blog(blog_id, context).await
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
