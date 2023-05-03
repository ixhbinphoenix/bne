#![allow(clippy::enum_variant_names)]
mod api;
mod api_wrapper;
mod database;
mod error;
mod models;
mod prelude;
mod utils;

use std::{
    collections::HashMap, env, fs, io::{self, BufReader}
};

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key}, middleware::Logger, web::{self, Data}, App, HttpResponse, HttpServer
};
use api::{check_session::check_session_get, get_timetable::get_timetable, login::login_post, register::register_post, get_lernbueros::get_lernbueros};
use database::surrealdb_repo::SurrealDBRepo;
use dotenv::dotenv;
use log::info;
use models::user_model::UserCRUD;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use crate::utils::env::{get_env, get_env_or};

#[derive(Clone)]
pub struct GlobalUntisData {
    school: String,
    subdomain: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let envv: HashMap<String, String> = env::vars().map(|(key, value)| (key, value)).collect();
    if cfg!(debug_assertions) {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    } else {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    }

    let config = load_rustls_config();

    info!("Connecting database...");

    let db_location = get_env_or("DB_LOCATION", "memory".to_string());
    let db_namespace = get_env_or("DB_NAMESPACE", "test".to_string());
    let db_database = get_env_or("DB_DATABASE", "test".to_string());

    let db_repo = SurrealDBRepo::init(db_location.clone(), db_namespace.clone(), db_database.clone())
        .await
        .expect("db-repo to connect");

    UserCRUD::init_table(db_repo.clone()).await.expect("table initilization to work");

    let school = get_env("UNTIS_SCHOOL");
    let subdomain = get_env("UNTIS_SUBDOMAIN");

    let untis_data = GlobalUntisData { school, subdomain };

    let cookie_key = if envv.contains_key("COOKIE_KEY") {
        Key::from(envv.get("COOKIE_KEY").unwrap().as_bytes())
    } else {
        Key::generate()
    };

    let port = get_env_or("PORT", "8080".to_string());

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(65536) // Fun fact: This is enough to fit the entire Bee movie script which
            // means it's probably way too much
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
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
                    .cookie_same_site(actix_web::cookie::SameSite::None)
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl_extension_policy(actix_session::config::TtlExtensionPolicy::OnStateChanges)
                            .session_ttl(Duration::days(7)),
                    )
                    .build(),
            )
            .wrap(cors)
            .app_data(json_config)
            .app_data(Data::new(db_repo.clone()))
            .app_data(Data::new(untis_data.clone()))
            .service(web::resource("/register").route(web::post().to(register_post)))
            .service(web::resource("/login").route(web::post().to(login_post)))
            .service(web::resource("/check_session").route(web::get().to(check_session_get)))
            .service(web::resource("/get_timetable").route(web::get().to(get_timetable)))
            .service(web::resource("/get_lernbueros").route(web::get().to(get_lernbueros)))
    })
    .bind_rustls(format!("127.0.0.1:{port}"), config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder().with_safe_defaults().with_no_client_auth();

    let cert_file = &mut BufReader::new(fs::File::open("cert.pem").expect("cert.pem to load"));
    let key_file = &mut BufReader::new(fs::File::open("key.pem").expect("key.pem to load"));

    let cert_chain = certs(cert_file).expect("certificate to load").into_iter().map(Certificate).collect();
    let mut keys: Vec<PrivateKey> =
        pkcs8_private_keys(key_file).expect("key to load").into_iter().map(PrivateKey).collect();

    if keys.is_empty() {
        panic!("Could not locate private keys");
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
