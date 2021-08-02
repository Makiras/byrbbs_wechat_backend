use actix_web::{get, middleware, App, HttpServer};
use once_cell::sync::Lazy;
use std::env;
use std::path::Path;

#[macro_use]
mod service;
mod config;

// Init config

pub static ARGS: Lazy<Vec<std::string::String>> = Lazy::new(|| env::args().collect());
pub static CONF: Lazy<config::Config> = Lazy::new(|| {
    match config::read_config(match ARGS.get(1) {
        Some(p) => Path::new(p),
        None => Path::new("config.toml"),
    }) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
});

#[get("/")]
async fn api_index() -> &'static str {
    return "Hello, This is byrbbs wechat app developed by BRYIO!\r\n";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("{}", Lazy::force(&CONF));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(api_index)
            .service(service::token::callback_get)
            .service(service::token::callback_post)
            .service(service::token::refresh)
    })
    .bind(
        CONF.server
            .host_ip
            .as_ref()
            .unwrap_or(&"127.0.0.1".to_string())
            .to_string()
            + &":".to_string()
            + &CONF.server.port.as_ref().unwrap_or(&8080).to_string(),
    )?
    .run()
    .await
}
