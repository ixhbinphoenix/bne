use actix_web::{error, web, HttpRequest, Responder, Result};
use serde::Serialize;

use crate::{
    models::{model::DBConnection, teacher_model::Teacher}, utils::env::get_env_unsafe
};

#[derive(Serialize)]
struct TeachersResponse {
    teachers: Vec<Teacher>,
}

pub async fn get_teachers(req: HttpRequest, db: web::Data<DBConnection>) -> Result<impl Responder> {
    let password = if let Some(session_cookie) = req.cookie("admin_password") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden("No Adminpassword".to_string()));
    };
    if password != get_env_unsafe("ADMIN_PASSWORD") {
        return Err(error::ErrorForbidden("Wrong Password".to_string()));
    }

    let teachers = Teacher::get_all(db).await?;
    Ok(web::Json(TeachersResponse { teachers }))
}
