use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::error;

use crate::{api::utils::TextResponse, database::sessions::delete_user_sessions, models::model::DBConnection};

pub async fn logout_all_post(id: Option<Identity>, db: web::Data<DBConnection>) -> Result<impl Responder> {
    if let Some(identity) = id {
        let id = match identity.id() {
            Ok(a) => a,
            Err(e) => {
                error!("There was an error trying to get id.id()\n{e}");
                return Err(error::ErrorInternalServerError( "Internal Server Error"));
            }
        };
        delete_user_sessions(db, id).await?;
        Ok(web::Json(TextResponse { message: "Logged out on all devices!".to_string()}))
    } else {
        Err(error::ErrorForbidden( "Not logged in!"))
    } 
}
