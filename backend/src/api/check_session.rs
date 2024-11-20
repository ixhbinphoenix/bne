use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::error;

use crate::api_wrapper::utils::TextResponse;



pub async fn check_session_get(id: Option<Identity>) -> Result<impl Responder> {
    if let Some(id) = id {
        match id.id() {
            Ok(_) => Ok(web::Json(TextResponse { message: "Authenticated".to_string()})),
            Err(e) => {
                error!("Error trying to get id.id()\n{}", e);
                Err(error::ErrorInternalServerError( "NOPE Server Error".to_string()).into())
            }
        }
    } else {
        Err(error::ErrorForbidden( "Not Authenticated".to_string()).into())
    }
}
