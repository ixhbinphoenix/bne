use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};

use crate::api::utils::TextResponse;

pub async fn logout_post(id: Option<Identity>) -> Result<impl Responder> {
    if let Some(id) = id {
        id.logout();
        Ok(web::Json(TextResponse {
            message: "Logout successful!".to_string(),
        }))
    } else {
        Err(error::ErrorForbidden("Not logged in!"))
    }
}
