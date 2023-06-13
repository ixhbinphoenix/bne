use std::str::FromStr;

use actix_web::{web, Responder, Result};
use serde::Deserialize;
use surrealdb::sql::Thing;
use log::{warn, error};
use uuid::Uuid;

use crate::{models::{links_model::{Link, LinkType}, model::{CRUD, ConnectionData}}, internalError, api::response::Response};

#[derive(Debug, Deserialize)]
pub struct UuidQuery {
    #[serde(rename = "type")]
    link_type: LinkType
}

// Path: /link/check_uuid/{uuid}?type={link_type}
pub async fn check_uuid_get(path: web::Path<String>, typequery: web::Query<UuidQuery>, db: ConnectionData) -> Result<impl Responder> {
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
            internalError!("There was a database error")
        }
    };

    if pot_link.is_none() {
        return Ok(Response::new_error(404, "Link not found".into()).into());
    }

    let link = pot_link.unwrap();

    if link.link_type != typequery.link_type {
        warn!("Link found but wrong type");
        return Ok(Response::new_error(404, "Link not found".into()).into());
    }

    Ok(web::Json(Response::new_success("Link found".to_string())))
}
