use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use log::error;

pub async fn check_session_get(id: Option<Identity>) -> impl Responder {
    if let Some(id) = id {
        match id.id() {
            Ok(id) => HttpResponse::Ok().body(format!("YEP {id}")),
            Err(e) => {
                error!("Error trying to get id.id()\n{}", e);
                HttpResponse::InternalServerError().body("NOPE Server Error".to_string())
            }
        }
    } else {
        HttpResponse::Forbidden().body("NOPE".to_string())
    }
}
