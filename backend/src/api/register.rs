use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use log::error;
use rand_core::OsRng;
use serde::Deserialize;

use crate::{
    api::response::Response,
    models::{
        model::{DBConnection, CRUD}, user_model::{User, UserCreate}
    }, prelude::Error, utils::password::valid_password
};

#[derive(Deserialize)]
pub struct RegisterData {
    email: String,
    password: String,
    person_id: i64,
    untis_cypher: String,
}

pub async fn register_post(
    data: web::Json<RegisterData>, db: web::Data<DBConnection>, request: HttpRequest,
) -> Result<impl Responder> {
    // TODO: Email validation
    let pot_user = User::get_from_email(db.clone(), data.email.clone()).await;
    if pot_user.is_ok() && pot_user.unwrap().is_some() {
        return Ok(web::Json(Response::new_error(403,"E-mail already associated to account!".to_string())));
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
            return Ok(Response::new_error(500, "Unknown error trying to hash password".to_string()).into())
        }
    };

    let db_user = UserCreate {
        email: data.email.clone(),
        person_id: data.person_id,
        password_hash,
        untis_cypher: data.untis_cypher.clone(),
    };

    let ret_user = match User::create(db, "users".to_owned(), db_user).await {
        Ok(a) => a,
        Err(e) => return Err(e.try_into()?),
    };

    match Identity::login(&request.extensions(), ret_user.id.to_string()) {
        Ok(_) => {}
        Err(e) => {
            error!("Error trying to log into Identity\n{}", e);
            return Ok(Response::new_error(500, "Error trying to login, please retry".to_string()).into());
        }
    };

    return Ok(Response::new_success("Account successfully registered".to_string()).into())
}
