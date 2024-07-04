use juniper::FieldResult;
use crate::graphql::Context;
use super::{ CreateUserDto, User };

pub trait IUsersService {
    async fn find_all(context: &Context) -> FieldResult<Vec<User>>;
    async fn find(id: i32, context: &Context) -> FieldResult<User>;
    async fn create_user(create_user_dto: CreateUserDto, context: &Context) -> FieldResult<User>;
    async fn find_by_email(email: String, context: &Context) -> FieldResult<User>;
}

pub struct UsersService {}

impl IUsersService for UsersService {
    async fn find_all(context: &Context) -> FieldResult<Vec<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&context.db).await
            .map_err(|e| {
                println!("{:?}", e);
                juniper::FieldError::new("Failed to fetch users", juniper::Value::Null)
            })
    }

    async fn find(id: i32, context: &Context) -> FieldResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&context.db).await
            .map_err(|e| {
                println!("{:?}", e);
                juniper::FieldError::new("No such user", juniper::Value::Null)
            })
    }

    async fn find_by_email(email:String,context:&Context) -> FieldResult<User>{
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&context.db).await
            .map_err(|e| {
                println!("{:?}", e);
                juniper::FieldError::new("No such user", juniper::Value::Null)
            })
    }

    async fn create_user(create_user_dto: CreateUserDto, context: &Context) -> FieldResult<User> {
        let CreateUserDto { email, password } = create_user_dto;
        let user = User {
            id: None,
            name: "name".to_string(),
            email,
            password,
            created_at: chrono::Utc::now().naive_utc(),
        };
        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, password, created_at) VALUES ($1, $2, $3, $4) RETURNING *"
        )
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.created_at)
            .fetch_one(&context.db).await
            .map_err(|e| {
                println!("{:?}", e);
                juniper::FieldError::new("Failed to create user", juniper::Value::Null)
            })
    }
}

