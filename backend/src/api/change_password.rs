use actix_identity::Identity;
use actix_web::{
    web::{self, Json}, Responder, Result
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Days, Utc};
use lettre::message::header::ContentType;
use log::{debug, error};
use rand_core::OsRng;
use serde::Deserialize;
use surrealdb::sql::Thing;

use super::response::Response;
use crate::{
    database::sessions::delete_user_sessions, mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
    }, prelude::Error, utils::password::valid_password
};

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
        return Ok(web::Json(Response::new_error(403, "Not logged in".into())));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => a,
        Err(e) => {
            error!("Error trying to get id\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
        }
    };

    let user = match User::get_from_id(db.clone(), Thing::from(id.split_once(':').unwrap())).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                return Ok(Response::new_error(500, "Internal Server Error".into()).into());
            }
        },
        Err(e) => {
            error!("Error trying to get user\n{e}");
            return Ok(Response::new_error(500, "Interal Server Error".into()).into());
        }
    };

    if let Err(e) = valid_password(&body.new_password) {
        return Err(Error::from(e).into());
    };

    if body.old_password == body.new_password {
        return Ok(Response::new_error(400, "New Password can't be Old Password".into()).into());
    }

    if user.verify_password(body.old_password.clone()).is_err() {
        debug!("Wrong password");
        return Ok(Response::new_error(403, "Wrong password".into()).into());
    }

    let argon2 = Argon2::default();
    let salt = SaltString::generate(OsRng);

    let hash = match argon2.hash_password(body.new_password.as_bytes(), &salt) {
        Ok(a) => a,
        Err(e) => {
            error!("Error hashing password\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
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
        return Ok(Response::new_error(500, "Internal Server Error".into()).into());
    }

    if let Err(e) = delete_user_sessions(db.clone(), new_user.clone().id.to_string()).await {
        error!("Error logging user out\n{e}");
        return Ok(Response::new_error(500, "Internal Server Error".into()).into());
    }

    let expiry_time = Utc::now().checked_add_days(Days::new(2)).unwrap();

    let reset_link = match Link::create_from_user(db, new_user.clone(), expiry_time, LinkType::PasswordReset).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating link\n{e}");
            return Ok(Response::new_error(500, "Error sending mail".into()).into());
        }
    };

    let template = match load_template("password_changed.html").await {
        Ok(a) => a.replace("${{RESET_URL}}", &reset_link),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Ok(Response::new_error(500, "Error sending mail".into()).into());
        }
    };

    let message =
        match build_mail(&new_user.clone().email, "Dein Passwort wurde geändert", ContentType::TEXT_HTML, template) {
            Ok(a) => a,
            Err(e) => {
                error!("Error building mail\n{e}");
                return Ok(Response::new_error(500, "Error sending mail".into()).into());
            }
        };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        return Ok(Response::new_error(500, "Error sending mail".into()).into());
    }

    Ok(web::Json(Response::new_success("Successfully changed Password".to_string())))
}
