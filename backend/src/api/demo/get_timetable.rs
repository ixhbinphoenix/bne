use actix_identity::Identity;
use actix_web::{Responder, web};
use serde::Serialize;

use crate::api::response::Response;

#[derive(Serialize)]
struct TimetableResponse {
    lessons: Vec<Lesson>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Lesson {
    teacher: String,
    lernbuero: bool,
    starts: u8,
    length: u8,
    day: u8,
    subject: String,
    room: String,
    subject_short: String
}

pub async fn get_timetable(id: Option<Identity>) -> impl Responder {
    if id.is_none() {
        return web::Json(Response::new_error(403, "Not logged in".to_string()));
    }
    return Response::new_success(TimetableResponse {
        lessons: vec![
            Lesson {
                teacher: "PPOW".to_string(),
                lernbuero: false,
                starts: 3,
                length: 2,
                day: 1,
                subject: "Informatik".to_string(),
                room: "O2-16NT".to_string(),
                subject_short: "IF".to_string()
            }
        ]
    }).into();
}
