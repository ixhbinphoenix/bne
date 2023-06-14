use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use log::error;
use serde::{Deserialize, Serialize};

use super::response::Response;
use crate::models::{model::DBConnection, user_model::User};

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    untis_cypher: String,
}

pub async fn login_post(
    data: web::Json<LoginData>, db: web::Data<DBConnection>, req: HttpRequest,
) -> Result<impl Responder> {
    let db_user: User = {
        // Very readable yes yes. Suprisingly clippy doesn't have a Problem with this
        match match User::get_from_email(db, data.email.clone()).await {
            Ok(n) => n,
            Err(e) => {
                error!("Unknown error occured when trying to get user.\n{}", e);
                return Ok(web::Json(Response::new_error(500, "Internal Server Error".to_owned())));
            }
        } {
            Some(u) => u,
            None => {
                return Ok(Response::new_error(403, "E-Mail or Password is incorrect!".to_owned()).into());
            }
        }
    };

    match db_user.verify_password(data.password.clone()) {
        Ok(_) => match Identity::login(&req.extensions(), db_user.id.to_string()) {
            Ok(_) => Ok(Response::<LoginResponse>::new_success(LoginResponse {
                untis_cypher: db_user.untis_cypher,
            })
            .into()),
            Err(e) => {
                error!("Error: Unknown error trying to login to Identity\n{}", e);
                Ok(Response::new_error(500, "Internal Server Error".to_owned()).into())
            }
        },
        Err(_) => Ok(Response::new_error(403, "E-Mail or Password is incorrect!".to_owned()).into()),
    }
}
