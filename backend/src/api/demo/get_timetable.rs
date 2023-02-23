use actix_identity::Identity;
use actix_web::{Responder, web};
use serde::Serialize;

use crate::{api::response::Response, api_wrapper::utils::FormattedLesson};

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
    subject_short: String,
    substitution: Option<Substitution>
}

#[derive(Serialize)]
struct Substitution {
    teacher: Option<String>,
    room: Option<String>,
    substition_text: Option<String>,
    cancelled: bool
}

impl Default for Substitution {
    fn default() -> Self {
        Self {
            teacher: None,
            room: None,
            substition_text: None,
            cancelled: false
        }
    }
}

impl Substitution {
    fn default_cancelled() -> Self {
        Self {
            teacher: None,
            room: None,
            substition_text: None,
            cancelled: true
        }
    }
}

impl From<FormattedLesson> for Lesson {
    fn from(value: FormattedLesson) -> Self {
        Self {
            teacher: value.teacher,
            lernbuero: value.is_lb,
            starts: value.start,
            length: value.length,
            day: value.day,
            subject: value.subject,
            room: value.room,
            subject_short: value.subject_short,
            // TODO: Get Substitution data from api_wrapper
            substitution: None
        }
    }
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
                subject_short: "IF".to_string(),
                substitution: Some(Substitution::default_cancelled())
            }
        ]
    }).into();
}
