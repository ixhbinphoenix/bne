use std::str::FromStr;

use actix_web::{error, web, Responder, Result};
use chrono::{Days, Utc};
use lettre::{message::header::ContentType, Address};
use log::{debug, error, warn};
use serde::Deserialize;
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{
    api::utils::TextResponse, database::sessions::delete_user_sessions, mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
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
        return Err(error::ErrorUnprocessableEntity("Not a valid e-mail"));
    }
    if Uuid::from_str(&path).is_err() {
        return Err(error::ErrorUnprocessableEntity("UUID is not a valid uuid"));
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
            return Err(error::ErrorInternalServerError("There was a database error"));
        }
    };

    if pot_link.is_none() {
        return Err(error::ErrorNotFound("Link not found"));
    }

    let link = pot_link.unwrap();

    match link.link_type {
        LinkType::EmailChange => {}
        _ => {
            // Potential Attacker really shouldn't know if there's a link of another type with the
            // provided UUID
            warn!("Link found but wrong type");
            return Err(error::ErrorNotFound("Link not found"));
        }
    }

    let user_id = link.user;

    let user = match User::get_from_id(db.clone(), user_id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User ID in link is not valid");
                return Err(error::ErrorInternalServerError("There was a database error"));
            }
        },
        Err(e) => {
            error!("Database error trying to get user from link\n{e}");
            return Err(error::ErrorInternalServerError("There was a database error"));
        }
    };

    if user.verify_password(body.password.clone()).is_err() {
        debug!("Client sent wrong password");
        return Err(error::ErrorForbidden("Wrong Password"));
    };

    if match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => a.is_some(),
        Err(e) => {
            error!("Getting potential user from mail failed\n{e}");
            return Err(error::ErrorInternalServerError("There was a database error"));
        }
    } {
        warn!("E-mail is already in use");
        return Err(error::ErrorForbidden("Mail already in use"));
    }

    let new_user = User {
        id: user_id.clone(),
        email: body.mail.clone(),
        password_hash: user.password_hash,
        person_id: user.person_id,
        untis_cypher: user.untis_cypher,
        verified: user.verified,
    };

    if let Err(e) = User::update_replace(db.clone(), user_id.clone(), new_user).await {
        error!("Error updating user email\n{e}");
        return Err(error::ErrorInternalServerError("There was a database error"));
    }

    if let Err(e) = Link::delete(db.clone(), link.id).await {
        warn!("Failed to delete link, ignoring\n{e}");
    }

    let updated_user = match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("Updated e-mail isn't found in the database?");
                return Err(error::ErrorInternalServerError("There was a database error"));
            }
        },
        Err(e) => {
            error!("Error trying to get updated user from database\n{e}");
            return Err(error::ErrorInternalServerError("There was a database error"));
        }
    };

    // Logout user from all devices
    if let Err(e) = delete_user_sessions(db.clone(), user_id.to_string()).await {
        error!("Error deleting user sessions\n{e}");
        return Err(error::ErrorInternalServerError("There was a database error"));
    };

    let expiry = Utc::now().checked_add_days(Days::new(2)).unwrap();

    let reset_link = match Link::create_from_user(db.clone(), updated_user, expiry, LinkType::EmailReset).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating reset link\n{e}");
            return Err(error::ErrorInternalServerError("There was an error sending out an e-mail"));
        }
    };

    let template = match load_template("email_changed.html").await {
        Ok(a) => a.replace("${{RESET_URL}}", &reset_link).replace("${{NEW_MAIL}}", &body.mail),
        Err(e) => {
            error!("Error loading mail template\n{e}");
            return Err(error::ErrorInternalServerError("There was an error sending out an e-mail"));
        }
    };

    let message =
        match build_mail(&user.email, "Deine E-Mail Addresse wurde geändert", ContentType::TEXT_HTML, template) {
            Ok(a) => a,
            Err(e) => {
                error!("Error constructing message\n{e}");
                return Err(error::ErrorInternalServerError("There was an error sending out an e-mail"));
            }
        };

    match send_mail(mailer, message).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error sending mail\n{e}");
            return Err(error::ErrorInternalServerError("There was an error sending out an e-mail"));
        }
    };

    Ok(web::Json(TextResponse {
        message: "Successfully updated e-mail".to_string(),
    }))
}
