mod db;
mod error;
mod jwt;
mod routes;
pub mod utils;

use crate::db::{DBConn, User};
use crate::jwt::JWT;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

struct Env {
    db_url: String,
    jwt_secret: String,
}

const DEFAULT_JWT_SECRET: &str = "secret";
const DEFAULT_LOG_LEVEL: &str = "info";

lazy_static! {
    static ref ENV: Env = {
        dotenvy::dotenv().ok();
        Env {
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string()),
        }
    };
}

#[derive(Clone)]
pub struct AppData {
    pub db_conn: DBConn,
    pub jwt: JWT,
    pub user: Arc<RwLock<Option<User>>>,
}

#[actix_web::main]
async fn main() -> error::Result<()> {
    let db_conn = DBConn::new(&ENV.db_url).await?;
    let jwt = JWT::new(&ENV.jwt_secret);
    let app_data = AppData {
        db_conn,
        jwt,
        user: Arc::new(RwLock::new(None)),
    };

    env_logger::init_from_env(env_logger::Env::default().default_filter_or(DEFAULT_LOG_LEVEL));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(Logger::default())
            .service(routes::user::sign_in)
            .service(routes::user::sign_up)
            .service(routes::user::get_current_user)
            .service(routes::user::update_user)
            .service(routes::transaction::create_transaction)
            .service(routes::transaction::get_transaction_by_id)
            .service(routes::transaction::get_transaction_list)
            .service(routes::account::create_account)
            .service(routes::account::get_account_detail)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
