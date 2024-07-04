use actix_web::{ App, HttpServer };
use db::establish_connection;
use handlers::register;
use std::io;
use actix_web::web::Data;
mod handlers;
mod graphql;
mod db;
mod services;
#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    let connection = establish_connection().await.unwrap();

    let context = graphql::Context {
        db: connection.to_owned(),
    };

    HttpServer::new(move || { App::new().app_data(Data::new(context.clone())).configure(register) })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
