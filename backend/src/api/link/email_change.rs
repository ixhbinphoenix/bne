use std::str::FromStr;

use actix_web::{web, Responder, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Days, Utc};
use lettre::{message::header::ContentType, Address};
use log::{debug, error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::response::Response, database::sessions::delete_user_sessions, mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::{User, UserPatch}
    }
};

#[derive(Deserialize)]
pub struct NewMail {
    mail: String,
    password: String,
}

// Konzept:
// 1. Mail anfordern über /change_mail mit Passwort im body
// 2. Über empfangene Mail E-Mail-Adresse ändern: /link/email_change/{uuid}
// 3. Sicherheitsmail an alte Adresse mit link zum zurücksetzen: /link/email_reset/{uuid}
// Path: /link/email_change/{uuid}
pub async fn email_change_post(
    path: web::Path<String>, body: web::Json<NewMail>, db: ConnectionData, mailer: web::Data<Mailer>,
) -> Result<impl Responder> {
    if body.mail.parse::<Address>().is_err() {
        return Ok(web::Json(Response::new_error(400, "Not a valid e-mail".into())));
    }
    if Uuid::from_str(&path).is_err() {
        return Ok(Response::new_error(400, "UUID is not a valid uuid".into()).into());
    }

    let pot_link = match Link::get_from_id(
        db.clone(),
        Thing {
            tb: "links".into(),
            id: path.into_inner().into(),
        },
    )
    .await
    {
        Ok(a) => a,
        Err(e) => {
            error!("There was an error getting a link from the database\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    };

    if pot_link.is_none() {
        return Ok(Response::new_error(404, "Link not found".into()).into());
    }

    let link = pot_link.unwrap();

    match link.link_type {
        LinkType::EmailChange => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Ok(Response::new_error(404, "Link not found".into()).into());
        }
    }

    let user_id = link.user;

    let user = match User::get_from_id(db.clone(), user_id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User ID in link is not valid");
                return Ok(Response::new_error(500, "There was a database error".into()).into());
            }
        },
        Err(e) => {
            error!("Database error trying to get user from link\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    };

    let argon2 = Argon2::default();

    let db_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            error!("Error: Stored hash is not a valid hash. User: {}", user.email);
            return Ok(Response::new_error(500, "Internal Server Error".to_owned()).into());
        }
    };

    if argon2.verify_password(body.password.as_bytes(), &db_hash).is_err() {
        debug!("Client sent wrong password");
        return Ok(Response::new_error(403, "Wrong Password".into()).into());
    };

    if match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => a.is_some(),
        Err(e) => {
            error!("Getting potential user from mail failed\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    } {
        warn!("E-mail is already in use");
        return Ok(Response::new_error(403, "Mail already in use".into()).into());
    }

    let new_user = UserPatch {
        id: user_id.clone(),
        email: Some(body.mail.clone()),
        password_hash: None,
        person_id: None,
        untis_cypher: None,
    };

    if User::update_merge(db.clone(), user_id.clone(), new_user).await.is_err() {
        error!("Error updating user email");
        return Ok(Response::new_error(500, "There was a database error".into()).into());
    }

    let updated_user = match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("Updated e-mail isn't found in the database?");
                return Ok(Response::new_error(500, "There was a database error".into()).into());
            }
        },
        Err(e) => {
            error!("Error trying to get updated user from database\n{e}");
            return Ok(Response::new_error(500, "There was a database error".into()).into());
        }
    };

    // Logout user from all devices
    if let Err(e) = delete_user_sessions(db.clone(), user_id.to_string()).await {
        error!("Error deleting user sessions\n{e}");
        return Ok(Response::new_error(500, "There was a database error".into()).into());
    };

    let expiry = Utc::now().checked_add_days(Days::new(2)).unwrap();

    let reset_link = match Link::create_from_user(db.clone(), updated_user, expiry, LinkType::EmailReset).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating reset link\n{e}");
            return Ok(Response::new_error(500, "There was an error sending out an e-mail".into()).into());
        }
    };

    let template = match load_template("email_changed.html").await {
        Ok(a) => a.replace("${{RESET_URL}}", &reset_link).replace("${{NEW_MAIL}}", &body.mail),
        Err(e) => {
            error!("Error loading mail template\n{e}");
            return Ok(Response::new_error(500, "There was an error sending out an e-mail".into()).into());
        }
    };

    let message = match build_mail(&body.mail, "Deine E-Mail Addresse wurde geändert", ContentType::TEXT_HTML, template)
    {
        Ok(a) => a,
        Err(e) => {
            error!("Error constructing message\n{e}");
            return Ok(Response::new_error(500, "There was an error sending out an e-mail".into()).into());
        }
    };

    match send_mail(mailer, message).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error sending mail\n{e}");
            return Ok(Response::new_error(500, "There was an error sending out an e-mail".into()).into());
        }
    };

    Ok(web::Json(Response::new_success("Successfully updated e-mail".to_string())))
}
