use actix_identity::Identity;
use actix_web::{web, Responder, Result, HttpResponse, HttpRequest, HttpMessage};
use argon2::{Argon2, PasswordVerifier, PasswordHash};
use serde::Deserialize;
use log::error;

use crate::{database::surrealdb_repo::SurrealDBRepo, models::user_model::{UserCRUD, User}, prelude::*};


#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String
}

pub async fn login_post(data: web::Json<LoginData>, db: web::Data<SurrealDBRepo>, req: HttpRequest, id: Option<Identity>) -> Result<impl Responder> {
    if id.is_some() {
        return Ok(HttpResponse::Forbidden()
                  .body(format!("403 Forbidden\nAlready logged in, log out first")))
    }
    let db_user: User = match UserCRUD::get_from_username(db, &data.username).await {
        Ok(n) => n,
        Err(e) => {
            match e {
                Error::ObjectNotFound(_) => {
                    return Ok(HttpResponse::Forbidden()
                        .body(format!("403 Forbidden\nUsername or Password is not correct!")))
                }
                _ => {
                    error!("Unknown error occured when trying to get user.\n{}", e);
                    return Ok(HttpResponse::InternalServerError()
                        .body(format!("500 Internal Server error")))
                }
            }
        },
    }.try_into()?;

    let argon2 = Argon2::default();

    let db_hash = match PasswordHash::new(&db_user.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            error!("Error: Stored hash is not a valid hash. User: {}", db_user.username);
            return Ok(HttpResponse::InternalServerError()
                      .body(format!("500 Internal Server error")))
        },
    };

    match argon2.verify_password(data.password.as_str().as_bytes(), &db_hash) {
        Ok(_) => {
            match Identity::login(&req.extensions(), db_user.id.expect("id to exist after conversion check")) {
                Ok(_) => {
                    Ok(HttpResponse::Ok().body("200 OK\nSuccessfully logged in"))
                },
                Err(e) => {
                    error!("Error: Unknown error trying to login to Identity\n{}", e);
                    Ok(HttpResponse::InternalServerError()
                       .body(format!("500 Internal Server error")))
                },
            }
        },
        Err(_) => {
            Ok(HttpResponse::Forbidden()
                      .body(format!("403 Forbidden\nUsername or Password is not correct!")))
        },
    }
}
