use actix_identity::Identity;
use actix_web::{Result, Responder, web};
use log::{error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::{models::{model::{CRUD, ConnectionData}, user_model::User}, internalError, api::response::Response, database::sessions::delete_user_sessions};

#[derive(Debug, Deserialize)]
pub struct DeleteBody {
    password: String
}

pub async fn delete_post(body: web::Json<DeleteBody>, id: Option<Identity>, db: ConnectionData) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in".into())));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => Thing::from(a.split_once(':').unwrap()),
        Err(e) => {
            error!("Error trying to get id\n{e}");
            internalError!()
        }
    };

    let user = match User::get_from_id(db.clone(), id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                internalError!()
            }
        },
        Err(e) => {
            error!("Error trying to get user\n{e}");
            internalError!()
        }
    };

    if user.verify_password(body.password.clone()).is_err() {
        warn!("Incorrect password");
        return Ok(web::Json(Response::new_error(403, "Incorrect Password".into())));
    }

    if let Err(e) = delete_user_sessions(db.clone(), id.to_string()).await {
        warn!("Couldn't log out, ignoring\n{e}");
    };

    if let Err(e) = User::delete(db, id).await {
        error!("Failed to delete account\n{e}");
        internalError!()
    };

    Ok(web::Json(Response::new_success("Deleted your Account, bye-bye!".to_string())))
}
