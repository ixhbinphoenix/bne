use std::str::FromStr;

use actix_web::{error, web, Responder, Result};
use log::{error, warn};
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::utils::TextResponse,
    models::{
        links_model::{Link, LinkType},
        model::{ConnectionData, CRUD},
        user_model::User,
    },
};

pub async fn verify_get(path: web::Path<String>, db: ConnectionData) -> Result<impl Responder> {
    if Uuid::from_str(&path).is_err() {
        return Err(error::ErrorUnprocessableEntity("UUID is not a valid uuid"));
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
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    if pot_link.is_none() {
        return Err(error::ErrorNotFound("Link not found"));
    }

    let link = pot_link.unwrap();

    match link.link_type {
        LinkType::VerifyAccount => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Err(error::ErrorNotFound("Link not found"));
        }
    }

    let user = match User::get_from_id(db.clone(), link.user.clone()).await {
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

    let new_user = User {
        id: user.id,
        email: user.email,
        password_hash: user.password_hash,
        untis_cypher: user.untis_cypher,
        person_id: user.person_id,
        verified: true,
    };

    if let Err(e) = User::update_replace(db.clone(), link.user, new_user).await {
        error!("Updating user failed\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    }

    if let Err(e) = Link::delete(db, link.id).await {
        warn!("Failed to delete link, ignoring\n{e}");
    }

    Ok(web::Json(TextResponse {
        message: "Successfully verified!".to_string(),
    }))
}
