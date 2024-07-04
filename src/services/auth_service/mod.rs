mod auth_service;

#[derive(Debug, juniper::GraphQLObject)]
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
