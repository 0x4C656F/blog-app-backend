use crate::graphql::{create_schema, Context, Schema};
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use dotenvy::var;
use jsonwebtoken::{decode, errors::Error as JwtError, DecodingKey, Validation};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
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
    println!("Secret: {:?}", &secret);
    let validation = Validation::default();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
async fn graphql_handler(
    schema: web::Data<Schema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
    auth_requirement: AuthRequirement,
) -> Result<HttpResponse, Error> {
    let user_id: Option<i32> = match auth_requirement {
        // TODO - remove the unwraps and think of smthing better
        AuthRequirement::Required => {
            let token = extract_token(&req)
                .ok_or_else(|| HttpResponse::Unauthorized().finish())
                .unwrap();

            let claims = validate_token(token)
                .map_err(|_| return HttpResponse::Unauthorized().finish())
                .unwrap();

            Some(claims.sub)
        }
        AuthRequirement::None => None,
    };

    let mut context = context.get_ref().clone();
    context.user_id = user_id;

    let res = data.execute(&schema, &context).await;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

async fn graphql_public(
    schema: web::Data<Schema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    graphql_handler(schema, context, data, req, AuthRequirement::None).await
}

async fn graphql_protected(
    schema: web::Data<Schema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    graphql_handler(schema, context, data, req, AuthRequirement::Required).await
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_schema()))
        .service(web::resource("/graphql/public").route(web::post().to(graphql_public)))
        .service(web::resource("/graphql/protected").route(web::post().to(graphql_protected)))
        .service(graphql_playground);
}
