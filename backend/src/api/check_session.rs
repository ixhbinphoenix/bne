use actix_identity::Identity;
use actix_web::{Responder, HttpResponse};

pub async fn check_session_get(id: Option<Identity>) -> impl Responder {
    if let Some(id) = id {
        match id.id() {
            Ok(id) => {
                HttpResponse::Ok().body(format!("YEP {id}"))
            },
            Err(_) => {
                HttpResponse::InternalServerError().body("NOPE Server Error".to_string())
            }
        }
    } else {
        HttpResponse::Forbidden().body("NOPE".to_string())
    }
}
