use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::{error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::{
    api::utils::TextResponse, database::sessions::delete_user_sessions, models::{
        model::{ConnectionData, CRUD}, user_model::User
    }
};

#[derive(Debug, Deserialize)]
pub struct DeleteBody {
    password: String,
}

pub async fn delete_post(
    body: web::Json<DeleteBody>, id: Option<Identity>, db: ConnectionData,
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden( "Not logged in"));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => Thing::from(a.split_once(':').unwrap()),
        Err(e) => {
            error!("Error trying to get id\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let user = match User::get_from_id(db.clone(), id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                return Err(error::ErrorInternalServerError("Internal Server Error"));
            }
        },
        Err(e) => {
            error!("Error trying to get user\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    if user.verify_password(body.password.clone()).is_err() {
        warn!("Incorrect password");
        return Err(error::ErrorForbidden( "Incorrect Password"));
    }

    if let Err(e) = delete_user_sessions(db.clone(), id.to_string()).await {
        warn!("Couldn't log out, ignoring\n{e}");
    };

    if let Err(e) = User::delete(db, id).await {
        error!("Failed to delete account\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    };

    Ok(web::Json(TextResponse { message: "Deleted your Account, bye-bye!".to_string()}))
}
