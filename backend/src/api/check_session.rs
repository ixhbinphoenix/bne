use actix_identity::Identity;
use actix_web::{web, Responder, Result};
use log::error;

use crate::api::response::Response;

pub async fn check_session_get(id: Option<Identity>) -> Result<impl Responder> {
    if let Some(id) = id {
        match id.id() {
            Ok(_) => Ok(web::Json(Response::new_success("Authenticated".to_string()))),
            Err(e) => {
                error!("Error trying to get id.id()\n{}", e);
                Ok(Response::new_error(500, "NOPE Server Error".to_string()).into())
            }
        }
    } else {
        Ok(Response::new_error(403, "Not Authenticated".to_string()).into())
    }
}
