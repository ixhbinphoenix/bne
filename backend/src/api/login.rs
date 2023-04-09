use actix_identity::Identity;
use actix_web::{web, Responder, Result, HttpRequest, HttpMessage};
use argon2::{Argon2, PasswordVerifier, PasswordHash};
use serde::{Deserialize, Serialize};
use log::error;

use crate::{database::surrealdb_repo::SurrealDBRepo, models::user_model::{UserCRUD, User}, prelude::*};

use super::response::Response;


#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    untis_cypher: String
}

pub async fn login_post(data: web::Json<LoginData>, db: web::Data<SurrealDBRepo>, req: HttpRequest, id: Option<Identity>) -> Result<impl Responder> {
    if id.is_some() {
        return Ok(web::Json(Response::new_error(403, "Already logged in! Log out first".to_owned())))
    }
    let db_user: User = match UserCRUD::get_from_username(db, &data.username).await {
        Ok(n) => n,
        Err(e) => {
            match e {
                Error::ObjectNotFound(_) => {
                    return Ok(Response::new_error(403, "Username or Password is incorrect!".to_owned()).into())
                }
                _ => {
                    error!("Unknown error occured when trying to get user.\n{}", e);
                    return Ok(Response::new_error(500, "Internal Server Error".to_owned()).into())
                }
            }
        },
    }.try_into()?;

    let argon2 = Argon2::default();

    let db_hash = match PasswordHash::new(&db_user.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            error!("Error: Stored hash is not a valid hash. User: {}", db_user.username);
            return Ok(Response::new_error(500, "Internal Server Error".to_owned()).into())
        },
    };

    match argon2.verify_password(data.password.as_str().as_bytes(), &db_hash) {
        Ok(_) => {
            match Identity::login(&req.extensions(), db_user.id.expect("id to exist after conversion check")) {
                Ok(_) => {
                    Ok(Response::<LoginResponse>::new_success(LoginResponse {
                        untis_cypher: db_user.untis_cypher.clone()
                    }).into())
                },
                Err(e) => {
                    error!("Error: Unknown error trying to login to Identity\n{}", e);
                    Ok(Response::new_error(500, "Internal Server Error".to_owned()).into())
                },
            }
        },
        Err(_) => {
            Ok(Response::new_error(403, "Username or Password is incorrect!".to_owned()).into())
        },
    }
}
