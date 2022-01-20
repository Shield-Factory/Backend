#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;
mod db;
mod errors;

use std::path::PathBuf;
use crate::handlers::app_config;
use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger, Error, HttpResponse, web, HttpRequest};
use tracing::{info, instrument};
use actix_web::http::StatusCode;
use actix_files::NamedFile;
use actix_web_static_files;
//use handlers::app_config;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("/home/caluri0/Documents/website_sf/src/html/home.html"))
    )
}

async fn login() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("/home/caluri0/Documents/website_sf/src/html/login.html"))
    )
}

async fn signup() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("/home/caluri0/Documents/website_sf/src/html/signup.html"))
    )
}


#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("Server configuation");
    
    let pool = config.db_pool().await
        .expect("Database configuration");

    let hashing = config.hashing();

    info!("Starting server at http://{}:{}/home.html", config.host, config.port);
    
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(actix_web_static_files::ResourceFiles::new(
                "/home/caluri0/Documents/website_sf/src/css", generated
            ))
            .wrap(Logger::default())
            .data(pool.clone())
            .data(hashing.clone())
            .configure(app_config)
            //.route("/{filename:.*}", web::get().to(index))
            .route("/home.html", web::get().to(home))
            .route("/login.html", web::get().to(login))
            .route("/signup.html", web::get().to(signup))
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    
    Ok(())
} 