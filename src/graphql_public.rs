use juniper::{ graphql_object, FieldResult, GraphQLObject };

use crate::{
    graphql::Context,
    services::auth_service::{ AuthService, IAuthService, SignInDto, SignUpDto, TokenPair },
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
    async fn sign_in(sign_in_dto: SignInDto, context: &Context) -> FieldResult<TokenPair> {
        println!("[{:?}] AuthService /sign_in with dto: {:?}", chrono::Utc::now(), &sign_in_dto);
        AuthService::login(sign_in_dto, context).await
    }
    async fn sign_up(sign_up_dto: SignUpDto, context: &Context) -> FieldResult<TokenPair> {
        println!("[{:?}] AuthService /sign_up with dto: {:?}", chrono::Utc::now(), &sign_up_dto);

        AuthService::register(sign_up_dto, context).await
    }
}
