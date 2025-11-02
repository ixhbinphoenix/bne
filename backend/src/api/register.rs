use actix_identity::Identity;
use actix_web::{error, web, HttpMessage, HttpRequest, Responder, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Months, Utc};
use lettre::{message::header::ContentType, Address};
use log::error;
use rand_core::OsRng;
use serde::Deserialize;

use crate::{
    api::utils::TextResponse, error::Error, mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::{DBConnection, CRUD}, user_model::{User, UserCreate}
    }, utils::password::valid_password
};

#[derive(Deserialize)]
pub struct RegisterData {
    email: String,
    password: String,
    person_id: i64,
    untis_cypher: String,
}

pub async fn register_post(
    data: web::Json<RegisterData>, db: web::Data<DBConnection>, request: HttpRequest, mailer: web::Data<Mailer>,
) -> Result<impl Responder> {
    if data.email.clone().parse::<Address>().is_err() {
        return Err(error::ErrorUnprocessableEntity("Not a valid email address"));
    }

    let pot_user = User::get_from_email(db.clone(), data.email.clone()).await;
    if pot_user.is_ok() && pot_user.unwrap().is_some() {
        return Err(error::ErrorForbidden("E-mail already associated to account!".to_string()));
    }
    if let Err(e) = valid_password(&data.password) {
        return Err(Error::from(e).into());
    };
    // A lot more checks coming not to worry
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = match argon2.hash_password(data.password.as_bytes(), &salt) {
        Ok(str) => str.to_string(),
        Err(e) => {
            error!("Error: Unknown error trying to hash password\n{}", e);
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let db_user = UserCreate {
        email: data.email.clone(),
        person_id: data.person_id,
        password_hash,
        untis_cypher: data.untis_cypher.clone(),
        verified: false,
    };

    let ret_user = match User::create(db.clone(), "users".to_owned(), db_user).await {
        Ok(a) => a,
        Err(e) => return Err(e.into()),
    };

    let expiry_time = Utc::now().checked_add_months(Months::new(1)).unwrap();

    let link = match Link::create_from_user(db, ret_user.clone(), expiry_time, LinkType::VerifyAccount).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating link\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let template = match load_template("verify.html").await {
        Ok(a) => a.replace("${{VERIFY_URL}}", &link),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let message = match build_mail(&ret_user.clone().email, "Accountverifizierung", ContentType::TEXT_HTML, template) {
        Ok(a) => a,

        Err(e) => {
            error!("Error building message\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    }

    if let Err(e) = Identity::login(&request.extensions(), ret_user.id.to_string()) {
        error!("Error trying to log into Identity\n{}", e);
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    };

    Ok(web::Json(TextResponse {
        message: "Account successfully registered".to_string(),
    }))
}
