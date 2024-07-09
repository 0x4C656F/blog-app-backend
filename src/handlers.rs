use crate::{
    graphql::Context,
    graphql_protected::{ create_protected_schema, ProtectedSchema },
    graphql_public::{ create_public_schema, PublicSchema },
};
use actix_web::{ get, web, Error, HttpRequest, HttpResponse, Responder };
use actix_web_lab::respond::Html;
use dotenvy::var;
use jsonwebtoken::{ decode, errors::Error as JwtError, DecodingKey, Validation };
use juniper::http::{ graphiql::graphiql_source, GraphQLRequest };
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: u64,
}
pub enum AuthRequirement {
    Required,
    None,
}

fn extract_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
}

fn validate_token(token: &str) -> Result<Claims, JwtError> {
    let secret = var("JWT_SECRET").unwrap();
    let validation = Validation::default();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation
    )?;

    Ok(token_data.claims)
}

enum Schema<'a> {
    Public(&'a PublicSchema),
    Protected(&'a ProtectedSchema),
}

async fn graphql_handler(
    schema: Schema<'_>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
    auth_requirement: AuthRequirement
) -> Result<HttpResponse, Error> {
    let user_id: Option<i32> = match auth_requirement {
        // TODO - remove the unwraps and think of smthing better
        AuthRequirement::Required => {
            let token = match extract_token(&req) {
                Some(token) => token,
                None => {
                    return Ok(HttpResponse::Unauthorized().finish());
                }
            };

            let claims = match validate_token(token) {
                Ok(claims) => claims,
                Err(_) => {
                    return Ok(HttpResponse::Unauthorized().finish());
                }
            };

            Some(claims.sub)
        }
        AuthRequirement::None => None,
    };

    let mut context = context.get_ref().clone();
    context.user_id = user_id;
    match schema {
        Schema::Public(schema) => {
            let res = data.execute(schema, &context).await;
            return Ok(HttpResponse::Ok().json(res));
        }
        Schema::Protected(schema) => {
            let res = data.execute(schema, &context).await;
            return Ok(HttpResponse::Ok().json(res));
        }
    }
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

async fn graphql_public(
    schema: web::Data<PublicSchema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    graphql_handler(
        Schema::Public(schema.as_ref()),
        context,
        data,
        req,
        AuthRequirement::None
    ).await
}

async fn graphql_protected(
    schema: web::Data<ProtectedSchema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    graphql_handler(
        Schema::Protected(schema.as_ref()),
        context,
        data,
        req,
        AuthRequirement::Required
    ).await
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_public_schema()))
        .app_data(web::Data::new(create_protected_schema()))
        .service(web::resource("/graphql/public").route(web::post().to(graphql_public)))
        .service(web::resource("/graphql/protected").route(web::post().to(graphql_protected)))
        .service(graphql_playground);
}
