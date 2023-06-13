use actix_web::{Result, Responder, web};
use actix_identity::Identity;
use log::error;
use surrealdb::sql::Thing;

use crate::{models::{model::{ConnectionData, CRUD}, user_model::User}, internalError};

use super::response::Response;

pub async fn verified_get(id: Option<Identity>, db: ConnectionData) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in!".to_string())));
    }

    let id = match id.unwrap().id() {
        Ok(a) => {
            Thing::from(a.split_once(':').unwrap())
        },
        Err(e) => {
            error!("Error getting id.id()\n{e}");
            internalError!()
        },
    };

    let user = match User::get_from_id(db, id).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                internalError!()
            },
        },
        Err(e) => {
            error!("Error getting user from id\n{e}");
            internalError!("There was a database error")
        },
    };

    Ok(web::Json(Response::new_success(user.verified)))
}
