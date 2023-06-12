// Konzept:
// 1. Passwort über Formular ändern: /password-change
// 2. Sicherheitsmail an E-Mail-Adresse mit link zum zurücksetzen senden
// 3. Passwort über link in E-Mail zurücksetzen: /link/password/{uuid}
use std::str::FromStr;

use actix_web::{web, Responder, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use log::{error, warn};
use rand_core::OsRng;
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::response::Response, database::sessions::delete_user_sessions, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
    }, prelude::Error, utils::password::valid_password
};

#[derive(Debug, Deserialize)]
pub struct PasswordChange {
    new_password: String,
}

// Path: /link/password/{uuid}
pub async fn reset_password_post(
    path: web::Path<String>, db: ConnectionData, body: web::Json<PasswordChange>,
) -> Result<impl Responder> {
    if Uuid::from_str(&path).is_err() {
        return Ok(Response::new_error(400, "UUID is not a valid uuid".into()).into());
    }
    if let Err(e) = valid_password(&body.new_password) {
        return Err(Error::from(e).try_into()?);
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
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    };

    if pot_link.is_none() {
        return Ok(Response::new_error(404, "Link not found".into()).into());
    }

    let link = pot_link.unwrap();

    match link.link_type {
        LinkType::PasswordReset => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Ok(Response::new_error(404, "Link not found".into()).into());
        }
    }

    let user_id = link.user;

    let user = match User::get_from_id(db.clone(), user_id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User ID in link is not valid");
                return Ok(Response::new_error(500, "There was a database error".into()).into());
            }
        },
        Err(e) => {
            error!("Database error trying to get user from link\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    };

    let argon2 = Argon2::default();

    let db_hash_str = user.clone().password_hash;
    let db_hash = match PasswordHash::new(&db_hash_str) {
        Ok(a) => a,
        Err(e) => {
            error!("Invalid hash in database\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    };

    if argon2.verify_password(body.new_password.as_bytes(), &db_hash).is_ok() {
        return Ok(Response::new_error(400, "New Password can't be Old Password".into()).into());
    }

    let salt = SaltString::generate(OsRng);
    let hash = match argon2.hash_password(body.new_password.as_bytes(), &salt) {
        Ok(a) => a.to_string(),
        Err(e) => {
            error!("Error trying to hash password\n{e}");
            return Ok(Response::new_error(500, "Unknown error trying to hash password".to_string()).into());
        }
    };

    let old_user = user.clone();
    let new_user = User {
        id: old_user.id,
        email: old_user.email,
        password_hash: hash,
        person_id: old_user.person_id,
        untis_cypher: old_user.untis_cypher,
    };

    if let Err(e) = User::update_replace(db.clone(), new_user.clone().id, new_user.clone()).await {
        error!("Error updating user\n{e}");
        return Ok(Response::new_error(500, "Internal Server Error".into()).into());
    }

    if let Err(e) = delete_user_sessions(db.clone(), new_user.id.to_string()).await {
        error!("Error logging user out\n{e}");
        return Ok(Response::new_error(500, "Internal Server Error".into()).into());
    }

    Ok(web::Json(Response::new_success("Successfully updated Password".to_string())))
}
