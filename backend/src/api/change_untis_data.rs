use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::{error, warn};
use serde::Deserialize;

use crate::{
    api::utils::TextResponse, database::sessions::delete_user_sessions, models::{
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
        return Err(error::ErrorForbidden( "Not logged in"));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => {
            let b = a.split_once(':').unwrap();
            (b.0.to_string(), b.1.to_string())
        },
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
        return Err(error::ErrorForbidden( "Incorrect Password".to_string()));
    }

    let new_user = User {
        id: user.id,
        email: user.email,
        password_hash: user.password_hash,
        verified: user.verified,
        untis_cypher: body.untis_cypher.clone(),
        person_id: body.person_id,
    };

    if let Err(e) = User::update_replace(db.clone(), new_user).await {
        error!("Error updating user\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    }

    if let Err(e) = delete_user_sessions(db, format!("{}:{}", id.0, id.1)).await {
        warn!("Error deleting user sessions, ignoring\n{e}");
    }

    Ok(web::Json(TextResponse { message: "Successfully changed Untis Data".to_string()}))
}
