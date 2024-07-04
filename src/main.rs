use actix_web::{ App, HttpServer };
use db::establish_connection;
use graphql::Context;
use handlers::register;
use std::io;
use actix_web::middleware::Logger;
use actix_web::web::Data;
mod handlers;
mod graphql;
mod db;
mod graphql_public;
mod graphql_protected;
mod services;

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    let connection = establish_connection().await.unwrap();

    let context = Context {
        db: connection.to_owned(),
        user_id: None
    };

    HttpServer::new(move || { App::new().wrap(Logger::default()).app_data(Data::new(context.clone())).configure(register) })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
