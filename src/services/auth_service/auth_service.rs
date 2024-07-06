use crate::{graphql::Context, handlers::Claims, services::users_service::*};
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{get_current_timestamp, EncodingKey, Header};
use juniper::FieldError;
use std::env::var;

use super::{LoginDto, RegisterDto, TokenPair};

pub trait IAuthService {
    async fn login(
        login_dto: LoginDto,
        context: &Context,
    ) -> juniper::FieldResult<super::TokenPair>;
    async fn register(
        register_dto: RegisterDto,
        context: &Context,
    ) -> juniper::FieldResult<super::TokenPair>;
    async fn refresh_token(
        refresh_token: String,
        context: &Context,
    ) -> juniper::FieldResult<super::TokenPair>;
}

pub struct AuthService {}

impl IAuthService for AuthService {
    async fn login(
        login_dto: LoginDto,
        context: &Context,
    ) -> juniper::FieldResult<super::TokenPair> {
        let LoginDto { password, email } = login_dto;

        let found_user = UsersService::find_by_email(email, context).await?;

        if bcrypt::verify(password, &found_user.password).unwrap() {
            return Err(FieldError::new(
                "Incorrect email or password",
                juniper::Value::Null,
            ));
        }
        AuthService::generate_token_pair(found_user.id.expect("User without id, wait what?"))
            .await
            .map_err(|_| {
                FieldError::new(
                    "Internal Server Error: token generation failed",
                    juniper::Value::Null,
                )
            })
    }
    async fn register(
        register_dto: RegisterDto,
        context: &Context,
    ) -> juniper::FieldResult<super::TokenPair> {
        todo!();
    }
    async fn refresh_token(
        refresh_token: String,
        context: &Context,
    ) -> juniper::FieldResult<super::TokenPair> {
        todo!();
    }
}

impl AuthService {
    async fn generate_token_pair(user_id: i32) -> Result<TokenPair, JwtError> {
        let secret = var("JWT_SECRET").unwrap();
        let access_token_claims = Claims {
            sub: user_id.clone(),
            exp: get_current_timestamp() + 3600,
        };
        let refresh_tokens_claims = Claims {
            sub: user_id,
            exp: get_current_timestamp() + 3600 * 24 * 7,
        };
        let access_token = jsonwebtoken::encode(
            &Header::default(),
            &access_token_claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        let refresh_token = jsonwebtoken::encode(
            &Header::default(),
            &refresh_tokens_claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
