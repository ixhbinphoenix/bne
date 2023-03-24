use actix_identity::Identity;
use actix_web::{Responder, web, Result};
use serde::{Serialize, Deserialize};

use crate::{api::response::Response, api_wrapper::{utils::{FormattedLesson, TimetableParameter}, untis_client::UntisClient}, prelude::Error, utils::time::{format_for_untis, get_this_monday, get_this_friday}};

#[derive(Serialize)]
struct TimetableResponse {
    lessons: Vec<FormattedLesson>
}

#[derive(Deserialize)]
pub struct TimetableQuery {
     from: Option<String>,
     until: Option<String>
}

pub async fn get_timetable(id: Option<Identity>, untis: web::Data<UntisClient>, query: web::Query<TimetableQuery>) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in".to_string())));
    }
    let from = match query.from.clone() {
        Some(from) => from,
        None => { format_for_untis(get_this_monday()) }
    };
    let until = match query.until.clone() {
        Some(until) => until,
        None => { format_for_untis(get_this_friday()) }
    };
    let timetable = match untis.clone().get_ref().to_owned().get_timetable(TimetableParameter::default(&untis.clone(), from, until)).await {
        Ok(timetable) => { timetable },
        Err(_) => {
            return Ok(Response::from(Error::UntisError).into());
        },
    };
    Ok(Response::new_success(TimetableResponse {
        lessons: timetable
    }).into())
}
