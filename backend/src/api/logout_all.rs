use actix_identity::Identity;
use actix_web::{web, Responder, Result};
use log::error;

use super::response::Response;
use crate::{database::sessions::delete_user_sessions, models::model::DBConnection};

pub async fn logout_all_post(id: Option<Identity>, db: web::Data<DBConnection>) -> Result<impl Responder> {
    if id.is_none() {
        Ok(web::Json(Response::new_error(403, "Not logged in!".into())))
    } else {
        let identity = id.unwrap();
        let id = match identity.id() {
            Ok(a) => a,
            Err(e) => {
                error!("There was an error trying to get id.id()\n{e}");
                return Ok(Response::new_error(500, "Internal Server Error".into()).into());
            }
        };
        delete_user_sessions(db, id).await?;
        Ok(Response::new_success("Logged out on all devices!").into())
    }
}
