use actix_identity::Identity;
use actix_web::{web, Responder, Result};

use super::response::Response;

pub async fn logout_post(id: Option<Identity>) -> Result<impl Responder> {
    if id.is_none() {
        Ok(web::Json(Response::new_error(403, "Not logged in!".into())))
    } else {
        let id = id.unwrap();
        id.logout();
        Ok(Response::new_success("Logout successful!").into())
    }
}
