mod db;
mod error;
mod graphql;
mod graphql_protected;
mod graphql_public;
mod handlers;
mod services;
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let connection = db::establish_connection().await.unwrap();
    dotenvy::dotenv().ok();
    let context = graphql::Context {
        db: connection.to_owned(),
        user_id: None,
    };
    actix_web::HttpServer
        ::new(move || {
            let cors = actix_cors::Cors::permissive();
            actix_web::App
                ::new()
                .wrap(cors)
                .app_data(actix_web::web::Data::new(context.clone()))
                .configure(handlers::register)
        })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
