use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use log::error;
use rand_core::OsRng;
use serde::Deserialize;

use crate::{
    database::surrealdb_repo::SurrealDBRepo, models::user_model::{User, UserCRUD}, prelude::Error, utils::password::valid_password
};

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    password: String,
    person_id: i64,
    untis_cypher: String,
}

pub async fn register_post(
    data: web::Json<RegisterData>, db: web::Data<SurrealDBRepo>, request: HttpRequest,
) -> Result<impl Responder> {
    if UserCRUD::get_from_username(db.clone(), &data.username).await.is_ok() {
        return Ok(HttpResponse::Forbidden().body("403 Forbidden\nUsername already taken!".to_string()));
    }
    if let Err(e) = valid_password(&data.password) {
        return Err(Error::from(e).try_into()?);
    };
    // A lot more checks coming not to worry
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = match argon2.hash_password(data.password.as_bytes(), &salt) {
        Ok(str) => str.to_string(),
        Err(e) => {
            error!("Error: Unknown error trying to hash password\n{}", e);
            return Ok(HttpResponse::Forbidden()
                .body("500 Internal Server Error\nUnknown error trying to hash password".to_string()));
        }
    };

    let db_user = User {
        id: None,
        username: data.username.clone(),
        person_id: data.person_id,
        password_hash,
        untis_cypher: data.untis_cypher.clone(),
    };

    let object = UserCRUD::create(db, db_user).await?;
    let ret_user: User = object.try_into()?;

    match Identity::login(&request.extensions(), ret_user.id.expect("id to be check during conversion")) {
        Ok(_) => {}
        Err(e) => {
            error!("Error trying to log into Identity\n{}", e);
            return Ok(HttpResponse::InternalServerError()
                .body("500 Internal Server Error\nError trying to login, please retry".to_string()));
        }
    };

    Ok(HttpResponse::Ok().body("200 OK".to_string()))
}
