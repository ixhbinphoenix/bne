#![allow(clippy::enum_variant_names)]
mod utils;
mod error;
mod prelude;
mod database;
mod api;
mod api_wrapper;
mod models;

use std::{io::{self, BufReader}, env, collections::HashMap, fs};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{HttpServer, middleware::Logger, web::{self, Data}, HttpResponse, App, cookie::{Key, time::Duration}};
use api::{login::login_post, check_session::check_session_get, register::register_post, demo::get_timetable::get_timetable};
use database::surrealdb_repo::SurrealDBRepo;
use dotenv::dotenv;
use models::user_model::UserCRUD;
use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let envv: HashMap<String, String> = env::vars().map(|(key, value)| (key, value)).collect();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = load_rustls_config(); 

    let db_location = if envv.contains_key("DB_LOCATION") { envv.get("DB_LOCATION").unwrap().clone() } else { "memory".to_string() };
    let db_namespace = if envv.contains_key("DB_NAMESPACE") { envv.get("DB_NAMESPACE").unwrap().clone() } else { "test".to_string() };
    let db_database = if envv.contains_key("DB_DATABASE") { envv.get("DB_DATABASE").unwrap().clone() } else { "test".to_string() };
    let db_repo = SurrealDBRepo::init(db_location.clone(), db_namespace.clone(), db_database.clone()).await.expect("db-repo to connect");

    UserCRUD::init_table(db_repo.clone()).await.expect("table initilization to work");

    let cookie_key = if envv.contains_key("COOKIE_KEY") { Key::from(envv.get("COOKIE_KEY").unwrap().as_bytes()) } else { Key::generate() };

    let port = if envv.contains_key("PORT") { envv.get("PORT").unwrap() } else { "8080" };

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(65536) // Fun fact: This is enough to fit the entire Bee movie script which
                          // means it's probably way too much
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        // This is not ok
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(logger)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), cookie_key.clone())
                    .cookie_same_site(actix_web::cookie::SameSite::Strict)
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .session_lifecycle(PersistentSession::default().session_ttl_extension_policy(actix_session::config::TtlExtensionPolicy::OnStateChanges).session_ttl(Duration::days(7)))
                    .build()
            )
            .wrap(cors)
            .app_data(json_config)
            .app_data(Data::new(db_repo.clone()))
            .service(web::resource("/register").route(web::post().to(register_post)))
            .service(web::resource("/login").route(web::post().to(login_post)))
            .service(web::resource("/check_session").route(web::get().to(check_session_get)))
            .service(
                web::scope("/demo")
                    .service(web::resource("/get_timetable").route(web::get().to(get_timetable)))
            )
    })
    .bind_rustls(format!("127.0.0.1:{port}"), config)?
    .run().await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(fs::File::open("cert.pem").expect("cert.pem to load"));
    let key_file = &mut BufReader::new(fs::File::open("key.pem").expect("key.pem to load"));

    let cert_chain = certs(cert_file).expect("certificate to load")
        .into_iter().map(Certificate).collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file).expect("key to load")
        .into_iter().map(PrivateKey).collect();

    if keys.is_empty() {
        println!("Could not locate private keys");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
