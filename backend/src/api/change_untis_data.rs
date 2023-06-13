use actix_identity::Identity;
use actix_web::{web, Responder, Result};
use log::error;
use serde::Deserialize;
use surrealdb::sql::Thing;

use super::response::Response;
use crate::{
    internalError, models::{
        model::{ConnectionData, CRUD}, user_model::User
    }
};

#[derive(Debug, Deserialize)]
pub struct UntisData {
    password: String,
    untis_cypher: String,
    person_id: i64,
}

pub async fn change_untis_data_post(
    body: web::Json<UntisData>, db: ConnectionData, id: Option<Identity>,
) -> Result<impl Responder> {
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
        return Ok(web::Json(Response::new_error(403, "Incorrect Password".to_string())));
    }

    let new_user = User {
        id: user.id,
        email: user.email,
        password_hash: user.password_hash,
        verified: user.verified,
        untis_cypher: body.untis_cypher.clone(),
        person_id: body.person_id,
    };

    if let Err(e) = User::update_replace(db, id, new_user).await {
        error!("Error updating user\n{e}");
        internalError!()
    }

    Ok(web::Json(Response::new_success("Successfully changed Untis Data".to_string())))
}
