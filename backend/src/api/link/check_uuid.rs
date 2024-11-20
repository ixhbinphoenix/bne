use std::str::FromStr;

use actix_web::{error, web, Responder, Result};
use log::{error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api_wrapper::utils::TextResponse, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}
    }
};

#[derive(Debug, Deserialize)]
pub struct UuidQuery {
    #[serde(rename = "type")]
    link_type: LinkType,
}

// Path: /link/check_uuid/{uuid}?type={link_type}
pub async fn check_uuid_get(
    path: web::Path<String>, typequery: web::Query<UuidQuery>, db: ConnectionData,
) -> Result<impl Responder> {
    if Uuid::from_str(&path).is_err() {
        return Err(error::ErrorUnprocessableEntity( "UUID is not a valid uuid"));
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
        return Err(error::ErrorNotFound( "Link not found"));
    }

    let link = pot_link.unwrap();

    if link.link_type != typequery.link_type {
        warn!("Link found but wrong type");
        return Err(error::ErrorNotFound( "Link not found"));
    }

    Ok(web::Json(TextResponse { message: "Link found".to_string()}))
}
