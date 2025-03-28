use actix_web::{error, web, HttpRequest, Responder, Result};
use serde::Serialize;

use crate::{
    models::{manual_lb_overwrite_model::ManualLBOverwrite, model::DBConnection},
    utils::env::get_env_unsafe,
};

#[derive(Serialize)]
struct LbResponse {
    lbs: Vec<ManualLBOverwrite>,
}

pub async fn get_manual_lb_overwrites(req: HttpRequest, db: web::Data<DBConnection>) -> Result<impl Responder> {
    let password = if let Some(session_cookie) = req.cookie("admin_password") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden("No Adminpassword".to_string()));
    };
    if password != get_env_unsafe("ADMIN_PASSWORD") {
        return Err(error::ErrorForbidden("Wrong Password".to_string()));
    }

    let lbs = ManualLBOverwrite::get_manual_lbs_overwrite(db).await?;
    Ok(web::Json(LbResponse { lbs }))
}
