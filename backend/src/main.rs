mod utils;
mod error;
mod prelude;
mod database;
mod api_wrapper;
mod models;

use std::{io, env, collections::HashMap};
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{HttpServer, middleware::Logger, web, HttpResponse, App, cookie::{Key, time::Duration}};
use database::surrealdb_repo::SurrealDBRepo;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let argv: HashMap<String, String> = env::vars().into_iter().map(|(key, value)| return (key, value)).collect();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("SslAcceptor to build");
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect("Key to load");
    builder.set_certificate_chain_file("cert.pem").expect("Certificate to load");


    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(65536) // Fun fact: This is enough to fit the entire Bee movie script which
                          // means it's probably way too much
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        let db_location = if argv.contains_key("DB_LOCATION") { argv.get("DB_LOCATION").unwrap().clone() } else { "memory".to_string() };
        let db_namespace = if argv.contains_key("DB_NAMESPACE") { argv.get("DB_NAMESPACE").unwrap().clone() } else { "test".to_string() };
        let db_database = if argv.contains_key("DB_DATABASE") { argv.get("DB_DATABASE").unwrap().clone() } else { "test".to_string() };
        let db_repo = SurrealDBRepo::init(db_location.clone(), db_namespace.clone(), db_database.clone());

        let cookie_key = if argv.contains_key("COOKIE_KEY") { Key::from(argv.get("COOKIE_KEY").unwrap().as_bytes()) } else { Key::generate() };

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(logger)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), cookie_key)
                    .cookie_same_site(actix_web::cookie::SameSite::Strict)
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .session_lifecycle(PersistentSession::default().session_ttl_extension_policy(actix_session::config::TtlExtensionPolicy::OnStateChanges).session_ttl(Duration::days(7)))
                    .build()
            )
            .app_data(json_config)
            .app_data(db_repo)
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run().await
}
