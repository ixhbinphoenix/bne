use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
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
    data: web::Json<LoginData>, db: web::Data<DBConnection>, req: HttpRequest, id: Option<Identity>,
) -> Result<impl Responder> {
    if id.is_some() {
        return Ok(web::Json(Response::new_error(403, "Already logged in! Log out first".to_owned())));
    }
    let db_user: User = {
        // Very readable yes yes. Suprisingly clippy doesn't have a Problem with this
        match match User::get_from_email(db, data.email.clone()).await {
            Ok(n) => n,
            Err(e) => {
                error!("Unknown error occured when trying to get user.\n{}", e);
                return Ok(Response::new_error(500, "Internal Server Error".to_owned()).into());
            }
        } {
            Some(u) => u,
            None => {
                return Ok(Response::new_error(500, "E-Mail or Password is incorrect!".to_owned()).into());
            }
        }
    };

    let argon2 = Argon2::default();

    let db_hash = match PasswordHash::new(&db_user.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            error!("Error: Stored hash is not a valid hash. User: {}", db_user.email);
            return Ok(Response::new_error(500, "Internal Server Error".to_owned()).into());
        }
    };

    match argon2.verify_password(data.password.as_str().as_bytes(), &db_hash) {
        Ok(_) => match Identity::login(&req.extensions(), db_user.id.to_string()) {
            Ok(_) => Ok(Response::<LoginResponse>::new_success(LoginResponse {
                untis_cypher: db_user.untis_cypher.clone(),
            })
            .into()),
            Err(e) => {
                error!("Error: Unknown error trying to login to Identity\n{}", e);
                Ok(Response::new_error(500, "Internal Server Error".to_owned()).into())
            }
        },
        Err(_) => Ok(Response::new_error(403, "Username or Password is incorrect!".to_owned()).into()),
    }
}
