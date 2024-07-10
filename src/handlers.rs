use crate::graphql::{ create_schema, Context, Schema };
use actix_web::{ get, web, Error, HttpRequest, HttpResponse, Responder };
use actix_web_lab::respond::Html;
use dotenvy::var;
use jsonwebtoken::{ decode, DecodingKey, Validation };
use juniper::http::{ graphiql::graphiql_source, GraphQLRequest };
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: u64,
}

fn extract_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()
        .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
}

fn validate_token(token: &str) -> Option<Claims> {
    let secret = var("JWT_SECRET").unwrap();
    let validation = Validation::default();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation
    ).ok();

    token_data.and_then(|data| { Some(data.claims) })
}

async fn graphql_handler(
    schema: web::Data<Schema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    let claims = extract_token(&req).and_then(validate_token);
    let user_id = claims.map(|cl| cl.sub);

    let mut context = context.get_ref().clone();
    context.user_id = user_id;

    let res = data.execute(&schema, &context).await;
    return Ok(HttpResponse::Ok().json(res));
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[get("/graphql/schema")]
async fn graphql_schema(schema: web::Data<Schema>) -> impl Responder {
    let schema = schema.as_ref().as_sdl();
    HttpResponse::Ok().json(schema)
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_schema()))
        .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
        .service(graphql_schema)
        .service(graphql_playground);
}
