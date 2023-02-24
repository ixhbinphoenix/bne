use actix_identity::Identity;
use actix_web::{Responder, web};
use serde::Serialize;

use crate::{api::response::Response, api_wrapper::utils::{FormattedLesson, Substitution}};

#[derive(Serialize)]
struct TimetableResponse {
    lessons: Vec<FormattedLesson>
}

pub async fn get_timetable(id: Option<Identity>) -> impl Responder {
    if id.is_none() {
        return web::Json(Response::new_error(403, "Not logged in".to_string()));
    }
    Response::new_success(TimetableResponse {
        lessons: vec![
            FormattedLesson {
                teacher: "PPOW".to_string(),
                is_lb: false,
                start: 3,
                length: 2,
                day: 1,
                subject: "Informatik".to_string(),
                room: "O2-16NT".to_string(),
                subject_short: "IF".to_string(),
                substitution: Substitution::default_cancelled()
            }
        ]
    }).into()
}
