use juniper::{ graphql_object, FieldResult };

use crate::{
    graphql::Context,
    services::auth_service::{ AuthService, IAuthService, LoginDto, RegisterDto, TokenPair },
};

pub struct PublicMutation;
pub struct PublicQuery;

#[graphql_object(context = Context)]
impl PublicQuery {
    async fn _dummy(context: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}
#[graphql_object(context = Context)]
impl PublicMutation {
    async fn login(login_dto: LoginDto, context: &Context) -> FieldResult<TokenPair> {
        AuthService::login(login_dto, context).await
    }
    async fn register(register_dto: RegisterDto, context: &Context) -> FieldResult<TokenPair> {
        AuthService::register(register_dto, context).await
    }
}

pub type PublicSchema = juniper::RootNode<
    'static,
    PublicQuery,
    PublicMutation,
    juniper::EmptySubscription<Context>
>;

pub fn create_public_schema() -> PublicSchema {
    PublicSchema::new(PublicQuery, PublicMutation, juniper::EmptySubscription::new())
}
