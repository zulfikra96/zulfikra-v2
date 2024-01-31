mod controllers;
mod routes;
mod config;

use actix_cors::Cors;
use actix_web::{http, App, HttpServer, web};
use controllers::home;
use dotenv::dotenv;
use crate::config::database::establish_connection;
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(move || {
        // Cors
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors) 
            .app_data(web::Data::new(establish_connection().clone()))
            .service(fs::Files::new("/assets","src/assets").show_files_listing())
            // Routes
            .service(home::index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}