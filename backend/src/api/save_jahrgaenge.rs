use actix_web::{error, web, HttpRequest, Responder, Result};

use crate::{
    models::{
        jahrgang_model::{Jahrgang, JahrgangCreate},
        model::DBConnection,
    },
    utils::env::get_env_unsafe,
};

use super::utils::TextResponse;

pub async fn save_jahrgaenge(
    data: web::Json<Vec<JahrgangCreate>>, req: HttpRequest, db: web::Data<DBConnection>,
) -> Result<impl Responder> {
    let password = if let Some(session_cookie) = req.cookie("admin_password") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden("No Adminpassword".to_string()));
    };
    if password != get_env_unsafe("ADMIN_PASSWORD") {
        return Err(error::ErrorForbidden("Wrong Password".to_string()));
    }

    Jahrgang::delete_all(db.clone()).await?;

    for jahrgang in data.clone().into_iter() {
        Jahrgang::insert_one(db.clone(), jahrgang).await?
    }

    Ok(web::Json(TextResponse {
        message: "Jahrgänge updated".to_string(),
    }))
}
