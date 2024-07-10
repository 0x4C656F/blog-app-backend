use juniper::graphql_object;
use juniper::EmptySubscription;
use juniper::FieldResult;
use sqlx::PgPool;
use crate::graphql_protected::ProtectedMutation;
use crate::graphql_protected::ProtectedQuery;
use crate::graphql_public::PublicMutation;
use crate::graphql_public::PublicQuery;
#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
    pub user_id: Option<i32>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn public() -> PublicQuery {
        PublicQuery
    }

    fn protected(context: &Context) -> FieldResult<ProtectedQuery> {
        if context.user_id.is_none() {
            return Err("Unauthorized".into());
        }
        Ok(ProtectedQuery)
    }
}
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn public() -> PublicMutation {
        PublicMutation
    }

    fn protected(context: &Context) -> FieldResult<ProtectedMutation> {
        if context.user_id.is_none() {
            return Err("Unauthorized".into());
        }
        Ok(ProtectedMutation)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
