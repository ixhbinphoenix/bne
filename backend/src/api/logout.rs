use actix_identity::Identity;
use actix_web::{web, Responder, Result};

use super::response::Response;

pub async fn logout_post(id: Option<Identity>) -> Result<impl Responder> {
    if let Some(id) = id {
        id.logout();
        Ok(Response::new_success("Logout successful!").into())
    } else {
        Ok(web::Json(Response::new_error(403, "Not logged in!".into())))
    } 
}
