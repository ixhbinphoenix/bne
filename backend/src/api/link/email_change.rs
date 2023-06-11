use std::str::FromStr;

use actix_web::{web, Responder, Result};
use lettre::Address;
use log::{error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::response::Response, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::{User, UserPatch}
    }
};

#[derive(Deserialize)]
pub struct NewMail {
    mail: String,
}

// Konzept:
// 1. Mail anfordern über /change-mail mit Passwort im body
// 2. Über empfangene Mail E-Mail-Adresse ändern: link/email-change/{uuid}
// 3. Sicherheitsmail an alte Adresse mit link zum zurücksetzen: link/email-reset/{uuid}
// Path: /link/email-change/{uuid}
pub async fn email_change_post(
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
        LinkType::EmailChange => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Ok(Response::new_error(404, "Link not found".into()).into());
        }
    }

    let user_id = link.user;

    let _ = match User::get_from_id(db.clone(), user_id.clone()).await {
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

    let new_user = UserPatch {
        id: user_id.clone(),
        email: Some(body.mail.clone()),
        password_hash: None,
        person_id: None,
        untis_cypher: None,
    };

    if User::update_merge(db, user_id, new_user).await.is_err() {
        error!("Error trying to update user email");
        return Ok(Response::new_error(500, "There was a database error".into()).into());
    }

    // TODO: Mail an alte e-mail das die geaendert wurde mit reset link

    Ok(web::Json(Response::new_success("Successfully update e-mail".to_string())))
}
