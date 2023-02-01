mod prelude;
mod database;
mod error;
mod api_wrapper;

use std::io;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{HttpServer, middleware::Logger, web, HttpResponse, App, cookie::{Key, time::Duration}};
use api_wrapper::untis_client::UntisClient;
use database::surrealdb_repo::SurrealDBRepo;



#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let client = UntisClient::init("Renkert_Marek_20060614".to_string(), "".to_string(), "theschedule".to_string(), "ges-münster".to_string(), "borys".to_string()).await.expect("Error creating the client");

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(65536) // Fun fact: This is enough to fit the entire Bee movie script which
                          // means it's probably way too much
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        let db_repo = SurrealDBRepo::init();

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(logger)
            .wrap(
                // This is NOT secure
                SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
                    .cookie_same_site(actix_web::cookie::SameSite::Strict)
                    .cookie_http_only(true)
                    .session_lifecycle(PersistentSession::default().session_ttl_extension_policy(actix_session::config::TtlExtensionPolicy::OnStateChanges).session_ttl(Duration::days(7)))
                    .build()
            )
            .app_data(json_config)
            .app_data(db_repo)
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}
