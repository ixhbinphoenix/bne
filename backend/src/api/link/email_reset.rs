use std::str::FromStr;

use actix_web::{error, web, Responder, Result};
use lettre::Address;
use log::{error, warn};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::utils::TextResponse, database::sessions::delete_user_sessions, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
    }
};

#[derive(Deserialize)]
pub struct NewMail {
    mail: String,
}

// Path: /link/email_reset/{uuid}
pub async fn email_reset_post(
    path: web::Path<String>, body: web::Json<NewMail>, db: ConnectionData,
) -> Result<impl Responder> {
    if body.mail.parse::<Address>().is_err() {
        return Err(error::ErrorUnprocessableEntity( "Not a valid e-mail"));
    }
    if Uuid::from_str(&path).is_err() {
        return Err(error::ErrorUnprocessableEntity( "UUID is not a valid uuid"));
    }

    let pot_link = match Link::get_from_id(
        db.clone(),
        ("links".into(), path.into_inner())
    )
    .await
    {
        Ok(a) => a,
        Err(e) => {
            error!("There was an error getting a link from the database\n{e}");
            return Err(error::ErrorInternalServerError( "There was a database error"));
        }
    };

    if pot_link.is_none() {
        return Err(error::ErrorNotFound( "Link not found"));
    }

    let link = pot_link.unwrap();

    match link.link_type {
        LinkType::EmailReset => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Err(error::ErrorNotFound( "Link not found"));
        }
    }

    let user_id = link.user;

    let user = match User::get_from_id(db.clone(), ("users".to_string(),user_id.clone())).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User ID in link is not valid");
                return Err(error::ErrorInternalServerError( "There was a database error"));
            }
        },
        Err(e) => {
            error!("Database error trying to get user from link\n{e}");
            return Err(error::ErrorInternalServerError( "There was a database error"));
        }
    };

    if match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => a.is_some(),
        Err(e) => {
            error!("Getting potential user from mail failed\n{e}");
            return Err(error::ErrorInternalServerError( "There was a database error"));
        }
    } {
        warn!("E-mail is already in use");
        return Err(error::ErrorForbidden( "Mail already in use"));
    }

    let new_user = User {
        id: user_id.clone(),
        email: body.mail.clone(),
        password_hash: user.password_hash,
        person_id: user.person_id,
        untis_cypher: user.untis_cypher,
        verified: user.verified,
    };

    if User::update_replace(db.clone(),  new_user).await.is_err() {
        error!("Error updating user email");
        return Err(error::ErrorInternalServerError( "There was a database error"));
    }

    if let Err(e) = Link::delete(db.clone(), ("links".to_string(),link.id)).await {
        warn!("Failed to delete link, ignoring\n{e}");
    }

    // Logout user from all devices
    if let Err(e) = delete_user_sessions(db.clone(), format!("{}", user_id)).await {
        error!("Error deleting user sessions\n{e}");
        return Err(error::ErrorInternalServerError( "There was a database error"));
    };

    Ok(web::Json(TextResponse { message: "Successfully updated e-mail".to_string()}))
}
