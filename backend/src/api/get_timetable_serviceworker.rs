use actix_identity::Identity;
use actix_web::{web::{self}, Responder, Result};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    api::response::Response, api_wrapper::{
        untis_client::UntisClient, utils::{FormattedLesson, TimetableParameter}
    }, models::{
        model::{DBConnection, CRUD}, user_model::User
    }, prelude::Error, utils::time::{format_for_untis, get_this_friday, get_this_monday}, GlobalUntisData
};

#[derive(Serialize)]
struct TimetableResponse {
    lessons: Vec<FormattedLesson>,
}

#[derive(Deserialize)]
pub struct TimetableQuery {
    from: Option<String>,
    until: Option<String>,
}

#[derive(Deserialize)]
pub struct ServiceWorkerQuery {
    jsessionid: Option<String>,
}

pub async fn get_timetable_serviceworker(
    id: Option<Identity>, query: web::Query<TimetableQuery>, data: web::Json<ServiceWorkerQuery>, untis_data: web::Data<GlobalUntisData>,
    db: web::Data<DBConnection>,
) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in".to_string())));
    }

    let jsessionid = match data.jsessionid.clone() {
        Some(session_cookie) => session_cookie,
        None => return Ok(Response::new_error(403, "No JSESSIONID provided".to_string()).into()),
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
                    return Ok(Response::new_error(500, "There was an error trying to get your id".to_string()).into());
                }
            }
            Err(e) => {
                error!("Error getting Identity id\n{e}");
                return Ok(Response::new_error(500, "There was an error trying to get your id".to_string()).into());
            }
        },
    )
    .await?;

    let user = match pot_user {
        Some(u) => u,
        None => {
            debug!("Deleted(?) User tried to log in with old session token");
            return Ok(Response::new_error(404, "This account doesn't exist!".to_string()).into());
        }
    };

    if !user.verified {
        return Ok(Response::new_error(
            403,
            "Account not verified! Check your E-Mails for a verification link".to_string(),
        )
        .into());
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
                return Ok(Response::new_error(400, "You done fucked up".into()).into());
            } else if let Error::UntisError(body) = e {
                return Ok(Response::new_error(500, "Untis done fucked up ".to_string() + &body).into());
            }
            else {
                return Ok(Response::new_error(500, "Some mysterious guy done fucked up".into()).into());
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

    let timetable = match untis.clone().get_timetable(TimetableParameter::default(untis, from, until)).await {
        Ok(timetable) => timetable,
        Err(err) => {
            return Ok(Response::new_error(500, "Untis done fucked up ".to_string() + &err.to_string()).into());
        }
    };
    Ok(Response::new_success(TimetableResponse { lessons: timetable }).into())
}
