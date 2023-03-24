use actix_identity::Identity;
use actix_web::{Responder, web, Result, HttpRequest};
use serde::{Serialize, Deserialize};
use log::error;

use crate::{api::response::Response, api_wrapper::{utils::{FormattedLesson, TimetableParameter}, untis_client::UntisClient}, prelude::Error, utils::time::{format_for_untis, get_this_monday, get_this_friday}, GlobalUntisData, models::user_model::{UserCRUD, User}, database::surrealdb_repo::SurrealDBRepo};

#[derive(Serialize)]
struct TimetableResponse {
    lessons: Vec<FormattedLesson>
}

#[derive(Deserialize)]
pub struct TimetableQuery {
     from: Option<String>,
     until: Option<String>
}

pub async fn get_timetable(id: Option<Identity>, query: web::Query<TimetableQuery>, req: HttpRequest, untis_data: web::Data<GlobalUntisData>, db: web::Data<SurrealDBRepo>) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in".to_string())));
    }

    let session_cookie = req.cookie("JSESSIONID");
    let jsessionid = if session_cookie.is_none() {
        return Ok(Response::new_error(403, "No JSESSIONID provided".to_string()).into());
    } else {
        session_cookie.unwrap().value().to_string()
    };

    let user: User = UserCRUD::get_from_id(db, match id.unwrap().id() {
        Ok(i) => i,
        Err(e) => {
            error!("Error getting Identity id\n{e}");
            return Ok(Response::new_error(500, "There was an error trying to get your id".to_string()).into());
        },
    }.as_str()).await?.try_into()?;

    let untis = match UntisClient::unsafe_init(jsessionid, user.person_id.try_into().expect("the database to not store numbers bigger than u16"), 5, "the-schedule".into(), untis_data.school.clone(), untis_data.subdomain.clone()).await {
        Ok(u) => u,
        Err(e) => {
            if e.is_request() {
                return Ok(Response::new_error(400, "You done fucked up".into()).into());
            } else {
                return Ok(Response::new_error(500, "Untis done fucked up".into()).into());
            }
        },
    };

    let from = match query.from.clone() {
        Some(from) => from,
        None => { format_for_untis(get_this_monday()) }
    };
    let until = match query.until.clone() {
        Some(until) => until,
        None => { format_for_untis(get_this_friday()) }
    };

    let timetable = match untis.clone().get_timetable(TimetableParameter::default(untis, from, until)).await {
        Ok(timetable) => timetable,
        Err(_) => {
            return Ok(Response::from(Error::UntisError).into());
        },
    };
    Ok(Response::new_success(TimetableResponse {
        lessons: timetable
    }).into())
}
