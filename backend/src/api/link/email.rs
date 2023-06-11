use std::str::FromStr;

use actix_web::{web, Responder, Result};
use lettre::Address;
use log::error;
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::response::Response, models::{
        links_model::Link, model::{ConnectionData, CRUD}, user_model::{User, UserPatch}
    }
};

#[derive(Deserialize)]
pub struct QueryParams {
    mail: String,
}

// Path: /link/email/{uuid}
pub async fn email_reset_get(
    path: web::Path<String>, query: web::Query<QueryParams>, db: ConnectionData,
) -> Result<impl Responder> {
    if query.mail.parse::<Address>().is_err() {
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
        email: Some(query.mail.clone()),
        password_hash: None,
        person_id: None,
        untis_cypher: None,
    };

    if User::update_merge(db, user_id, new_user).await.is_err() {
        error!("Error trying to update user email");
        return Ok(Response::new_error(500, "There was a database error".into()).into());
    }

    // TODO: Syxn gibt mir nen 'Your email has been reset' template

    Ok(web::Json(Response::new_success("Successfully update e-mail".to_string())))
}
