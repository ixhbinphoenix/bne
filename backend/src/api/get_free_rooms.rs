use actix_identity::Identity;
use actix_web::{error, web, HttpRequest, Responder, Result};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    api_wrapper::{
        untis_client::UntisClient,
        utils::{FormattedFreeRoom, TimetableParameter},
    },
    error::Error,
    models::{
        model::{DBConnection, CRUD},
        user_model::User,
    },
    utils::time::{format_for_untis, get_this_friday, get_this_monday},
    GlobalUntisData,
};

#[derive(Serialize)]
struct TimetableResponse {
    rooms: Vec<FormattedFreeRoom>,
}

#[derive(Deserialize)]
pub struct TimetableQuery {
    from: Option<String>,
    until: Option<String>,
}

pub async fn get_free_rooms(
    id: Option<Identity>, query: web::Query<TimetableQuery>, req: HttpRequest, untis_data: web::Data<GlobalUntisData>,
    db: web::Data<DBConnection>,
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden("Not logged in".to_string()));
    }

    let jsessionid = if let Some(session_cookie) = req.cookie("JSESSIONID") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden("No JSESSIONID provided".to_string()));
    };

    let pot_user: Option<User> = User::get_from_id(
        db.clone(),
        match id.unwrap().id() {
            Ok(i) => {
                let split = i.split_once(':');
                if split.is_some() {
                    Thing::from(split.unwrap())
                } else {
                    error!("ID in session_cookie is wrong???");
                    return Err(error::ErrorInternalServerError(
                        "There was an error trying to get your id".to_string(),
                    ));
                }
            }
            Err(e) => {
                error!("Error getting Identity id\n{e}");
                return Err(error::ErrorInternalServerError("There was an error trying to get your id".to_string()));
            }
        },
    )
    .await?;

    let user = match pot_user {
        Some(u) => u,
        None => {
            debug!("Deleted(?) User tried to log in with old session token");
            return Err(error::ErrorNotFound("This account doesn't exist!".to_string()));
        }
    };

    if !user.verified {
        return Err(error::ErrorUnauthorized("Account not verified! Check your E-Mails for a verification link"));
    }

    let untis = match UntisClient::unsafe_init(
        jsessionid,
        user.person_id.try_into().expect("the database to not store numbers bigger than u16"),
        5,
        "the-schedule".into(),
        untis_data.school.clone(),
        untis_data.subdomain.clone(),
        db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            if let Error::Reqwest(_) = e {
                return Err(error::ErrorUnprocessableEntity("Request could not be processed"));
            } else {
                return Err(error::ErrorInternalServerError(e.to_string()));
            }
        }
    };

    let from = match query.from.clone() {
        Some(from) => from,
        None => format_for_untis(get_this_monday()),
    };
    let until = match query.until.clone() {
        Some(until) => until,
        None => format_for_untis(get_this_friday()),
    };

    let rooms = match untis.clone().get_free_rooms(TimetableParameter::default(untis, from, until)).await {
        Ok(rooms) => rooms,
        Err(_err) => {
            return Err(error::ErrorInternalServerError("Error while fetching rooms"));
        }
    };
    Ok(web::Json(TimetableResponse { rooms }))
}
