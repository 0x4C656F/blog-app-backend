use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use db::establish_connection;
use dotenvy::dotenv;
use graphql::Context;
use handlers::register;
use std::io;
mod db;
mod graphql;
mod graphql_protected;
mod graphql_public;
mod handlers;
mod services;

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    let connection = establish_connection().await.unwrap();
    dotenv().ok();
    let context = Context {
        db: connection.to_owned(),
        user_id: None,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(context.clone()))
            .configure(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
