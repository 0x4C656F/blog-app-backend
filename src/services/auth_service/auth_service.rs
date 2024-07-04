use juniper::FieldError;
use crate::{graphql::Context, services::users_service::*};

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
        let LoginDto {password, email} = login_dto;

        let found_user = UsersService::find_by_email(email, context).await?;    
        
        if bcrypt::verify(password, &found_user.password).unwrap(){
            return Err(FieldError::new("Incorrect email or password", juniper::Value::Null))
        }




        todo!();
    }
    async fn register(
        register_dto: RegisterDto,
        context: &Context
    ) -> juniper::FieldResult<super::TokenPair> {
        todo!();
    }

}
