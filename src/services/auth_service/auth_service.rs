use crate::graphql::Context;

use super::{ LoginDto, RegisterDto };

pub trait IAuthService {
    async fn login(
        login_dto: LoginDto,
        context: &Context
    ) -> juniper::FieldResult<super::TokenPair>;
    async fn register(
        register_dto: RegisterDto,
        context: &Context
    ) -> juniper::FieldResult<super::TokenPair>;
}

pub struct AuthService {}

impl IAuthService for AuthService {
    async fn login(
        login_dto: LoginDto,
        context: &Context
    ) -> juniper::FieldResult<super::TokenPair> {
        todo!();
    }
    async fn register(
        register_dto: RegisterDto,
        context: &Context
    ) -> juniper::FieldResult<super::TokenPair> {
        todo!();
    }
}
