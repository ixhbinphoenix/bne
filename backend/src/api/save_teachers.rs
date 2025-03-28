use actix_identity::Identity;
use actix_web::{error, web, HttpRequest, Responder, Result};

use crate::{models::{model::DBConnection}, teacher_model::{Teacher, TeacherCreate}, utils::env::get_env_unsafe};

use super::utils::TextResponse;


pub async fn save_teachers(
    data: web::Json<Vec<TeacherCreate>>,
    req: HttpRequest,
    db: web::Data<DBConnection>
) -> Result<impl Responder> {
    let password = if let Some(session_cookie) = req.cookie("admin_password") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden( "No Adminpassword".to_string()));
    };
    if password != get_env_unsafe("ADMIN_PASSWORD") {
        return Err(error::ErrorForbidden("Wrong Password".to_string()))
    }

    Teacher::delete_all(db.clone()).await?;

    for teacher in data.clone().into_iter() {
        Teacher::insert_one(db.clone(), teacher).await?
    }


    Ok(web::Json(TextResponse {message: "Teachers updated".to_string()}))
}