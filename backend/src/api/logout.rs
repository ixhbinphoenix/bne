use actix_identity::Identity;
use actix_web::{Result, Responder, web};

use super::response::Response;

pub async fn logout_post(id: Option<Identity>) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in!".into())));
    } else {
        let id = id.unwrap();
        id.logout();
        return Ok(Response::new_success("Logout successful!").into());
    }
}
