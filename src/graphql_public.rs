use juniper::graphql_object;

use crate::graphql::Context;

pub struct PublicMutation;
pub struct PublicQuery;

#[graphql_object(context = Context)]
impl PublicQuery {
    fn hello() -> &'static str {
        "Hello, World!"
    }

 
}#[graphql_object(context = Context)]
impl PublicMutation {
    fn hello() -> &'static str {
        "Hello, World!"
    }
}