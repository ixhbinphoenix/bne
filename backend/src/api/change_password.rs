use actix_identity::Identity;
use actix_web::{
    error, web::{self, Json}, Responder, Result
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Days, Utc};
use lettre::message::header::ContentType;
use log::{debug, error};
use rand_core::OsRng;
use serde::Deserialize;

use crate::{
    database::sessions::delete_user_sessions, error::Error, mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
    }, utils::password::valid_password
};
use crate::api::utils::TextResponse;

#[derive(Debug, Deserialize)]
pub struct PasswordChange {
    old_password: String,
    new_untis_cypher: String,
    new_password: String,
}

pub async fn change_password_post(
    body: Json<PasswordChange>, id: Option<Identity>, db: ConnectionData, mailer: web::Data<Mailer>,
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden( "Not logged in"));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => {
            let b = a.split_once(':').unwrap();
            (b.0.to_string(), b.1.to_string())
        },
        Err(e) => {
            error!("Error trying to get id\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let user = match User::get_from_id(db.clone(), id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                return Err(error::ErrorInternalServerError( "Internal Server Error"));
            }
        },
        Err(e) => {
            error!("Error trying to get user\n{e}");
            return Err(error::ErrorInternalServerError( "Interal Server Error"));
        }
    };

    if let Err(e) = valid_password(&body.new_password) {
        return Err(Error::from(e).into());
    };

    if body.old_password == body.new_password {
        return Err(error::ErrorUnprocessableEntity( "New Password can't be Old Password"));
    }

    if user.verify_password(body.old_password.clone()).is_err() {
        debug!("Wrong password");
        return Err(error::ErrorForbidden( "Wrong password"));
    }

    let argon2 = Argon2::default();
    let salt = SaltString::generate(OsRng);

    let hash = match argon2.hash_password(body.new_password.as_bytes(), &salt) {
        Ok(a) => a,
        Err(e) => {
            error!("Error hashing password\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let old_user = user.clone();

    let new_user = User {
        id: user.id,
        email: user.email,
        password_hash: hash.to_string(),
        person_id: user.person_id,
        untis_cypher: body.new_untis_cypher.clone(),
        verified: user.verified,
    };

    if let Err(e) = User::update_replace(db.clone(), old_user.id, new_user.clone()).await {
        error!("Error updating user\n{e}");
        return Err(error::ErrorInternalServerError( "Internal Server Error"));
    }

    let new_user_id = new_user.clone().id;
    if let Err(e) = delete_user_sessions(db.clone(), format!("{}:{}", new_user_id.0, new_user_id.1)).await {
        error!("Error logging user out\n{e}");
        return Err(error::ErrorInternalServerError( "Internal Server Error"));
    }

    let expiry_time = Utc::now().checked_add_days(Days::new(2)).unwrap();

    let reset_link = match Link::create_from_user(db, new_user.clone(), expiry_time, LinkType::PasswordReset).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating link\n{e}");
            return Err(error::ErrorInternalServerError( "Error sending mail"));
        }
    };

    let template = match load_template("password_changed.html").await {
        Ok(a) => a.replace("${{RESET_URL}}", &reset_link),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Err(error::ErrorInternalServerError( "Error sending mail"));
        }
    };

    let message =
        match build_mail(&new_user.clone().email, "Dein Passwort wurde geändert", ContentType::TEXT_HTML, template) {
            Ok(a) => a,
            Err(e) => {
                error!("Error building mail\n{e}");
                return Err(error::ErrorInternalServerError( "Error sending mail"));
            }
        };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        return Err(error::ErrorInternalServerError( "Error sending mail"));
    }

    Ok(web::Json(TextResponse { message: "Successfully changed Password".to_string()}))
}
