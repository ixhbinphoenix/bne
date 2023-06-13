#![allow(clippy::enum_variant_names)]
mod api;
mod api_wrapper;
mod database;
mod error;
mod mail;
mod models;
mod prelude;
mod utils;

use std::{
    collections::HashMap, env, fs, io::{self, BufReader}
};

use actix_cors::Cors;
use actix_identity::{config::LogoutBehaviour, IdentityMiddleware};
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_session_surrealdb::SurrealSessionStore;
use actix_web::{
    cookie::{time::Duration, Key}, middleware::Logger, web::{self, Data}, App, HttpResponse, HttpServer
};
use api::{
    change_email::change_email_get, change_password::change_password_post, change_untis_data::change_untis_data_post, check_session::check_session_get, delete::delete_post, forgot_password::forgot_password_post, get_lernbueros::get_lernbueros, get_timetable::get_timetable, link::{
        email_change::email_change_post, email_reset::email_reset_post, password::reset_password_post, verify::verify_get
    }, login::login_post, logout::logout_post, logout_all::logout_all_post, register::register_post, verified::verified_get
};
use dotenv::dotenv;
use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism}, AsyncSmtpTransport, Tokio1Executor
};
use log::info;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use crate::{
    mail::utils::Mailer, models::{links_model::Link, model::CRUD, user_model::User}, utils::env::{get_env, get_env_or}
};

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

    let db_location = get_env_or("DB_LOCATION", "127.0.0.1:8000".to_string());

    let db = Surreal::new::<Ws>(db_location.clone()).await.expect("DB to connect");

    let db_user = get_env_or("DB_USERNAME", "root".to_string());
    let db_pass = get_env_or("DB_PASSWORD", "root".to_string());

    info!("Signing in...");

    db.signin(Root {
        username: db_user.as_str(),
        password: db_pass.as_str(),
    })
    .await
    .expect("DB Credentials to be correct");

    let db_namespace = get_env_or("DB_NAMESPACE", "test".to_string());
    let db_database = get_env_or("DB_DATABASE", "test".to_string());

    db.use_ns(db_namespace.clone()).use_db(db_database.clone()).await.expect("using namespace and db to work");

    User::init_table(db.clone()).await.expect("Table initialization to work");

    Link::init_table(db.clone()).await.expect("Table initialization to work");

    let session_db = Surreal::new::<Ws>(db_location).await.expect("DB to connect");

    session_db
        .signin(Root {
            username: db_user.as_str(),
            password: db_pass.as_str(),
        })
        .await
        .expect("DB Credentials to be correct");

    session_db.use_ns(db_namespace).use_db(db_database).await.expect("using namespace and db to work");

    info!("Connecting SMTP...");

    let smtp_username = get_env("MAIL_USERNAME");
    let smtp_password = get_env("MAIL_PASSWORD");
    let creds = Credentials::new(smtp_username, smtp_password);

    let smtp_server = get_env("MAIL_SERVER");
    let mailer: Mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)
        .expect("SMTP Server to connect")
        .credentials(creds)
        .authentication(vec![Mechanism::Plain])
        .build::<Tokio1Executor>();

    info!("SMTP Connected!");

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
            .allowed_origin(if cfg!(debug_assertions) {
                "http://localhost:3000"
            } else {
                "https://theschedule.de"
            })
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(IdentityMiddleware::builder().logout_behaviour(LogoutBehaviour::PurgeSession).build())
            .wrap(logger)
            .wrap(
                SessionMiddleware::builder(
                    SurrealSessionStore::from_connection(session_db.clone(), "sessions"),
                    cookie_key.clone(),
                )
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
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(mailer.clone()))
            .app_data(Data::new(untis_data.clone()))
            .service(web::resource("/register").route(web::post().to(register_post)))
            .service(web::resource("/login").route(web::post().to(login_post)))
            .service(web::resource("/logout").route(web::post().to(logout_post)))
            .service(web::resource("/logout_all").route(web::post().to(logout_all_post)))
            .service(web::resource("/delete").route(web::post().to(delete_post)))
            .service(web::resource("/check_session").route(web::get().to(check_session_get)))
            .service(web::resource("/get_timetable").route(web::get().to(get_timetable)))
            .service(web::resource("/get_lernbueros").route(web::get().to(get_lernbueros)))
            .service(web::resource("/change_email").route(web::get().to(change_email_get)))
            .service(web::resource("/change_password").route(web::post().to(change_password_post)))
            .service(web::resource("/forgot_password").route(web::post().to(forgot_password_post)))
            .service(web::resource("/change_untis_data").route(web::post().to(change_untis_data_post)))
            .service(web::resource("/verified").route(web::get().to(verified_get)))
            .service(
                web::scope("/link")
                    .service(web::resource("/email_change/{uuid}").route(web::post().to(email_change_post)))
                    .service(web::resource("/email_reset/{uuid}").route(web::post().to(email_reset_post)))
                    .service(web::resource("/password/{uuid}").route(web::post().to(reset_password_post)))
                    .service(web::resource("/verify/{uuid}").route(web::get().to(verify_get))),
            )
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
