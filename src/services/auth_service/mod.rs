mod auth_service;
pub use auth_service::*;
#[derive(Debug, juniper::GraphQLObject, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
}

pub fn get_user_id_or_throw(context: &crate::graphql::Context) -> Result<(), juniper::FieldError> {
    let token = context
        .user_id
        .as_ref()
        .ok_or_else(|| juniper::FieldError::new("Unauthorized", juniper::Value::Null))?;
    Ok(())
}
