use std::env;

use actix_web::{App, HttpServer, web};
use db::{DbConnectionParameters, init_db_connection};

mod api;
mod auth;
mod db;
mod password;
mod response;
mod service;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();

    let db = web::Data::new(
        init_db_connection(DbConnectionParameters {
            host: env::var("SQLDB_HOST").unwrap(),
            port: env::var("SQLDB_PORT").unwrap().parse::<u32>().unwrap(),
            database: env::var("SQLDB_DATABASE").unwrap(),
            user: env::var("SQLDB_USER").unwrap(),
            password: env::var("SQLDB_PASSWORD").unwrap(),
        })
        .await
        .expect("Database connection could not be initialized."),
    );

    HttpServer::new(move || {
        App::new()
            // -- db --
            .app_data(db.clone())
            // -- auth --
            .service(
                web::scope("/auth")
                    .service(api::auth::get_me_from_access_token)
                    .service(api::auth::create_access_token)
                    .service(api::auth::refresh_access_token),
            )
            // -- api --
            .service(
                web::scope("/api")
                    .service(api::whoami)
                    // -- v1 --
                    .service(
                        web::scope("/v1")
                            // -- -- account --
                            .service(api::v1::account::create_account)
                            .service(api::v1::account::create_roles)
                            .service(api::v1::account::add_roles_to_account),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
