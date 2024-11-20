use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::error;
use surrealdb::sql::Thing;

use crate::models::{
        model::{ConnectionData, CRUD}, user_model::User
    };

pub async fn verified_get(id: Option<Identity>, db: ConnectionData) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden( "Not logged in!".to_string()));
    }

    let id = match id.unwrap().id() {
        Ok(a) => Thing::from(a.split_once(':').unwrap()),
        Err(e) => {
            error!("Error getting id.id()\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let user = match User::get_from_id(db, id).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                return Err(error::ErrorInternalServerError("Internal Server Error"));
            }
        },
        Err(e) => {
            error!("Error getting user from id\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    Ok(web::Json(user.verified))
}
