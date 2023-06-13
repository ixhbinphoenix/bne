use actix_web::{web, Responder, Result};
use chrono::{Days, Utc};
use lettre::{message::header::ContentType, Address};
use log::error;
use serde::Deserialize;

use super::response::Response;
use crate::{
    mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::ConnectionData, user_model::User
    }
};

#[derive(Debug, Deserialize)]
pub struct ForgotPassword {
    mail: String,
}

pub async fn forgot_password_post(
    body: web::Json<ForgotPassword>, db: ConnectionData, mailer: web::Data<Mailer>,
) -> Result<impl Responder> {
    if body.mail.parse::<Address>().is_err() {
        return Ok(web::Json(Response::new_error(400, "Not a valid e-mail".into())));
    }

    let user = match User::get_from_email(db.clone(), body.mail.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                return Ok(Response::new_error(404, "No account associated with e-mail".into()).into());
            }
        },
        Err(e) => {
            error!("Error getting user from mail\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
        }
    };

    let expiry_time = Utc::now().checked_add_days(Days::new(2)).unwrap();

    let link = match Link::create_from_user(db, user.clone(), expiry_time, LinkType::PasswordReset).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating link\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
        }
    };

    let template = match load_template("password_reset.html").await {
        Ok(a) => a.replace("${{RESET_URL}}", &link),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
        }
    };

    let message = match build_mail(&body.mail.clone(), "Passwort Zurücksetzen", ContentType::TEXT_HTML, template) {
        Ok(a) => a,
        Err(e) => {
            error!("Error building mail\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
        }
    };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        return Ok(Response::new_error(500, "Internal Server Error".into()).into());
    }

    Ok(web::Json(Response::new_success("Sent E-Mail, check your Inbox".to_string())))
}
