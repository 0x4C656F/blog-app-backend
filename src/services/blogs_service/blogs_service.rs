use juniper::{ graphql_value, FieldError, FieldResult };
use crate::graphql::Context;
use super::{ Blog, CreateBlogDto };
use crate::error::ToFieldError;
pub trait IBlogsService {
    async fn blogs(context: &Context) -> FieldResult<Vec<Blog>>;
    async fn blogs_by_user_id(user_id: i32, context: &Context) -> FieldResult<Vec<Blog>>;
    async fn create_blog(create_blog_dto: CreateBlogDto, context: &Context) -> FieldResult<Blog>;
    async fn publish_blog(blog_id: i32, context: &Context) -> FieldResult<bool>;
    async fn blog(blog_id: i32, context: &Context) -> FieldResult<Blog>;
}

pub struct BlogsService {}
impl IBlogsService for BlogsService {
    async fn blogs(context: &Context) -> FieldResult<Vec<Blog>> {
        sqlx::query_as::<_, Blog>("SELECT * FROM blogs")
            .fetch_all(&context.db).await
            .to_field_error("Failed to fetch blogs")
    }

    async fn blogs_by_user_id(user_id: i32, context: &Context) -> FieldResult<Vec<Blog>> {
        sqlx::query_as::<_, Blog>("SELECT * FROM blogs WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&context.db).await
            .to_field_error("Failed to fetch blogs")
    }

    async fn create_blog(create_blog_dto: CreateBlogDto, context: &Context) -> FieldResult<Blog> {
        let CreateBlogDto { title, content } = create_blog_dto;
        let blog = Blog {
            id: None,
            title,
            content,
            user_id: 1,
            published: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        sqlx::query_as(
            "INSERT INTO blogs (title, content, user_id, published, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
            .bind(&blog.title)
            .bind(&blog.content)
            .bind(&blog.user_id)
            .bind(&blog.published)
            .bind(&blog.created_at)
            .bind(&blog.updated_at)
            .fetch_one(&context.db).await
            .to_field_error("Failed to create blog")
    }

    async fn publish_blog(blog_id: i32, context: &Context) -> FieldResult<bool> {
        let res = sqlx
            ::query("UPDATE blogs SET published = true WHERE id = $1")
            .bind(blog_id)
            .execute(&context.db).await;
        if let Ok(result) = res {
            if result.rows_affected() == 0 {
                return Err(FieldError::new("No such blog", graphql_value!({ "id": blog_id })));
            }
            Ok(true)
        } else {
            Err(FieldError::new("Failed to publish blog", graphql_value!({ "id": blog_id })))
        }
    }

    async fn blog(blog_id: i32, context: &Context) -> FieldResult<Blog> {
        sqlx::query_as::<_, Blog>("SELECT * FROM blogs WHERE id = $1")
            .bind(blog_id)
            .fetch_one(&context.db).await
            .to_field_error("No such blog")
    }
}
