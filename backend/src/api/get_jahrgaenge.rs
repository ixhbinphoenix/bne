use actix_web::{error, web, HttpRequest, Responder, Result};
use serde::Serialize;

use crate::{
    models::{jahrgang_model::Jahrgang, model::DBConnection},
    utils::env::get_env_unsafe,
};

#[derive(Serialize)]
struct JahrgaengeResponse {
    jahrgaenge: Vec<Jahrgang>,
}

pub async fn get_jahrgaenge(req: HttpRequest, db: web::Data<DBConnection>) -> Result<impl Responder> {
    let password = if let Some(session_cookie) = req.cookie("admin_password") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden("No Adminpassword".to_string()));
    };
    if password != get_env_unsafe("ADMIN_PASSWORD") {
        return Err(error::ErrorForbidden("Wrong Password".to_string()));
    }
    let jahrgaenge = Jahrgang::get_jahrgaenge(db).await?;
    Ok(web::Json(JahrgaengeResponse { jahrgaenge }))
}
