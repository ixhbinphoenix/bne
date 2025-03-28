use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::{debug, error};
use serde::Serialize;
use surrealdb::sql::Thing;

use crate::models::{manual_lb_model::ManualLB, model::{DBConnection, CRUD}, user_model::User};

#[derive(Serialize)]
struct LbResponse {
    lbs: Vec<ManualLB>,
}

pub async fn get_manual_lbs(
    id: Option<Identity>,
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

    let lbs = ManualLB::get_manual_lbs(db).await?;
    Ok(web::Json(LbResponse { lbs }))
}