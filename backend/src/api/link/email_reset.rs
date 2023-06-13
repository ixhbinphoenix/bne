use std::str::FromStr;

use actix_web::{web, Responder, Result};
use lettre::Address;
use log::{error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::response::Response, database::sessions::delete_user_sessions, models::{
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
        return Ok(web::Json(Response::new_error(400, "Not a valid e-mail".into())));
    }
    if Uuid::from_str(&path).is_err() {
        return Ok(Response::new_error(400, "UUID is not a valid uuid".into()).into());
    }

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
        LinkType::EmailReset => {}
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

    if match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => a.is_some(),
        Err(e) => {
            error!("Getting potential user from mail failed\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    } {
        warn!("E-mail is already in use");
        return Ok(Response::new_error(403, "Mail already in use".into()).into());
    }

    let new_user = User {
        id: user_id.clone(),
        email: body.mail.clone(),
        password_hash: user.password_hash,
        person_id: user.person_id,
        untis_cypher: user.untis_cypher,
        verified: user.verified
    };

    if User::update_replace(db.clone(), user_id.clone(), new_user).await.is_err() {
        error!("Error updating user email");
        return Ok(Response::new_error(500, "There was a database error".into()).into());
    }

    // Logout user from all devices
    if let Err(e) = delete_user_sessions(db.clone(), user_id.to_string()).await {
        error!("Error deleting user sessions\n{e}");
        return Ok(Response::new_error(500, "There was a database error".into()).into());
    };

    Ok(web::Json(Response::new_success("Successfully updated e-mail".to_string())))
}
