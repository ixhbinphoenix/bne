use actix_web::{error, web, HttpRequest, Responder, Result};

use super::utils::TextResponse;
use crate::{
    models::{
        manual_lb_model::{ManualLB, ManualLBCreate}, model::DBConnection
    }, utils::env::get_env_unsafe
};

pub async fn save_manual_lbs(
    data: web::Json<Vec<ManualLBCreate>>, req: HttpRequest, db: web::Data<DBConnection>,
) -> Result<impl Responder> {
    let password = if let Some(session_cookie) = req.cookie("admin_password") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden("No Adminpassword".to_string()));
    };
    if password != get_env_unsafe("ADMIN_PASSWORD") {
        return Err(error::ErrorForbidden("Wrong Password".to_string()));
    }

    ManualLB::delete_all(db.clone()).await?;

    for lb in data.clone().into_iter() {
        ManualLB::insert_one(db.clone(), lb).await?
    }

    Ok(web::Json(TextResponse {
        message: "Teachers updated".to_string(),
    }))
}
