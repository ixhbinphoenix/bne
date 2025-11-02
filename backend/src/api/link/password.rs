// Konzept:
// 1. Passwort über Formular ändern: /password-change
// 2. Sicherheitsmail an E-Mail-Adresse mit link zum zurücksetzen senden
// 3. Passwort über link in E-Mail zurücksetzen: /link/password/{uuid}
use std::str::FromStr;

use actix_web::{error, web, Responder, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use log::{error, warn};
use rand_core::OsRng;
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::utils::TextResponse, database::sessions::delete_user_sessions, error::Error, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
    }, utils::password::valid_password
};

#[derive(Debug, Deserialize)]
pub struct PasswordChange {
    new_password: String,
    new_untis_cypher: String,
    new_person_id: i64,
}

// Path: /link/password/{uuid}
pub async fn reset_password_post(
    path: web::Path<String>, db: ConnectionData, body: web::Json<PasswordChange>,
) -> Result<impl Responder> {
    if Uuid::from_str(&path).is_err() {
        return Err(error::ErrorUnprocessableEntity("UUID is not a valid uuid"));
    }
    if let Err(e) = valid_password(&body.new_password) {
        return Err(Error::from(e).into());
    };

    let pot_link = match Link::get_from_id(
        db.clone(),
        Thing {
            tb: "links".into(),
            id: path.into_inner().into(),
        },
    )
    .await
    {
        Ok(a) => a,
        Err(e) => {
            error!("There was an error getting a link from the database\n{e}");
            return Err(error::ErrorInternalServerError("There was a database error"));
        }
    };

    if pot_link.is_none() {
        return Err(error::ErrorNotFound("Link not found"));
    }

    let link = pot_link.unwrap();

    match link.link_type {
        LinkType::PasswordReset => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Err(error::ErrorNotFound("Link not found"));
        }
    }

    let user_id = link.user;

    let user = match User::get_from_id(db.clone(), user_id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User ID in link is not valid");
                return Err(error::ErrorInternalServerError("There was a database error"));
            }
        },
        Err(e) => {
            error!("Database error trying to get user from link\n{e}");
            return Err(error::ErrorInternalServerError("There was a database error"));
        }
    };

    let argon2 = Argon2::default();

    if user.verify_password(body.new_password.clone()).is_ok() {
        return Err(error::ErrorUnprocessableEntity("New Password can't be Old Password"));
    }

    let salt = SaltString::generate(OsRng);
    let hash = match argon2.hash_password(body.new_password.as_bytes(), &salt) {
        Ok(a) => a.to_string(),
        Err(e) => {
            error!("Error trying to hash password\n{e}");
            return Err(error::ErrorInternalServerError("Unknown error trying to hash password".to_string()));
        }
    };

    let old_user = user.clone();
    let new_user = User {
        id: old_user.id,
        email: old_user.email,
        password_hash: hash,
        person_id: body.new_person_id,
        untis_cypher: body.new_untis_cypher.clone(),
        verified: old_user.verified,
    };

    if let Err(e) = User::update_replace(db.clone(), new_user.clone().id, new_user.clone()).await {
        error!("Error updating user\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    }

    if let Err(e) = Link::delete(db.clone(), link.id).await {
        warn!("Failed to delete link, ignoring\n{e}");
    }

    if let Err(e) = delete_user_sessions(db.clone(), new_user.id.to_string()).await {
        error!("Error logging user out\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    }

    Ok(web::Json(TextResponse {
        message: "Successfully updated Password".to_string(),
    }))
}
