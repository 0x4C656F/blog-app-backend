use juniper::{graphql_object, FieldResult};

use crate::{
    graphql::Context,
    services::auth_service::{AuthService, IAuthService, LoginDto, RegisterDto, TokenPair},
};

pub struct PublicMutation;
pub struct PublicQuery;

#[graphql_object(context = Context)]
impl PublicQuery {
    fn hello() -> &'static str {
        "Hello, World!"
    }
}
#[graphql_object(context = Context)]
impl PublicMutation {
    fn hello() -> &'static str {
        "Hello, World!"
    }
    async fn login(login_dto: LoginDto, context: &Context) -> FieldResult<TokenPair> {
        AuthService::login(login_dto, context).await
    }
    async fn register(register_dto: RegisterDto, context: &Context) -> FieldResult<TokenPair> {
        AuthService::register(register_dto, context).await
    }
}
