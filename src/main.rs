
mod note;
mod auth;
pub(crate) use auth::{generate_token, validate_token};
mod transaction;

use auth_macros::jwt_guard;
#[cfg(test)]
mod test;
mod utils;
mod middleware;
mod server;
mod account;


use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App,Error, HttpServer, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use dotenv::{dotenv, from_filename};
use log::info;
use moka::future::Cache;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::account::login_handler;
use self::middleware::SignerValidator;
use self::server::MyWebSocket;



// WebSocket handshake and start `MyWebSocket` actor.
async fn websocket(req: HttpRequest, stream: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    println!("✅Connection to the ws a connect!");
    ws::start(MyWebSocket::new(data), &req, stream)
}

pub struct AppState {
    db: Pool<Postgres>,
    local_cache: LocalCache<String>,
    redis: RedisCache,
}

#[derive(Default)]
pub struct  LocalCache<V: AsRef<str>>{
    db: Arc<HashMap<String, Cache<String, V>>>
}

#[derive(Clone)]
pub struct  RedisCache{
    db: Arc<redis::Client>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "actix_web=info,promote_node=info"); }
    }
    // dotenv().ok();
    env_logger::init();
    parse_env()?;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let redis_host = std::env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_port = std::env::var("REDIS_PORT").expect("REDIS_PORT must be set");
    let redis_password = std::env::var("REDIS_PASSWORD");

    let client =if let Ok(pass) =redis_password{
        redis::Client::open(format!("redis://:{}@{}:{}/5", pass, redis_host, redis_port))
        .expect("Failed to create Redis client")
    }else{
        redis::Client::open(format!("redis://{}:{}/5", redis_host, redis_port))
        .expect("Failed to create Redis client")
    };

    let redis_client = RedisCache{
        db: Arc::new(client)
    };

    println!("🚀 Server started successfully");

   let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone(), local_cache: Default::default(), redis: redis_client.clone()}))
            .route("/login",  web::post().to(login_handler))
            .service( web::resource("/ws").route(web::get().to(websocket)))
            .configure(config)
            .wrap(cors)
            .wrap(SignerValidator)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8001));
     match server {
         Ok(s) => {
             s.run().await.expect("server start fail");
         },
         Err(err) => {
             println!("🔥 Failed to start the server: {:?}", err);
             std::process::exit(1);
         },
     };
    Ok(())

}

fn parse_env() -> std::io::Result<()> {
    let environment = env::var("ENV").unwrap_or_else(|_| "default".to_string());
    // 根据 `ENV` 环境变量值选择相应的 .env 文件
    match environment.as_str() {
        "local" => {
            from_filename(".local.env").ok();
            info!("Loaded local.env");
        },
        "production" => {
            from_filename(".production.env").ok();
            info!("Loaded production.env");
        },
        "staging" => {
            from_filename(".staging.env").ok();
            info!("Loaded staging.env");
        },
        "dev" => {
            from_filename(".dev.env").ok();
            info!("Loaded .dev.env");
        },
        _ => {
            dotenv().ok(); // 默认加载 .env 文件
            info!("Loaded .env");
        },
    }
    Ok(())
}

fn config(cfg: &mut web::ServiceConfig) {
    let api =  web::scope("/v1");
    let api=  note::config(api);
    let api = transaction::config(api);
    cfg.service(api);
}
