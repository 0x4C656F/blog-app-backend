use juniper::{ graphql_object, FieldResult, GraphQLObject };

use crate::{
    graphql::Context,
    services::auth_service::{ AuthService, IAuthService, LoginDto, RegisterDto, TokenPair },
};

#[derive(GraphQLObject)]
struct MockObject {
    field: String,
}

pub struct PublicMutation;
pub struct PublicQuery;

#[graphql_object(context = Context)]
impl PublicQuery {
    async fn dummy(_context: &Context) -> FieldResult<MockObject> {
        Ok(MockObject { field: "dummy".to_string() })
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
