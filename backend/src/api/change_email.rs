use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use chrono::{DateTime, Days, Utc};
use lettre::message::header::ContentType;
use log::error;
use surrealdb::sql::Thing;


use crate::{
    api_wrapper::utils::TextResponse, mail::{
        mailing::{build_mail, send_mail}, utils::{load_template, Mailer}
    }, models::{
        links_model::{Link, LinkType}, model::{ConnectionData, CRUD}, user_model::User
    }
};

pub async fn change_email_get(
    id: Option<Identity>, db: ConnectionData, mailer: web::Data<Mailer>,
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden( "Not logged in"));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => a,
        Err(e) => {
            error!("Error trying to get id.id()\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let user = match User::get_from_id(db.clone(), Thing::from(id.split_once(':').unwrap())).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                return Err(error::ErrorInternalServerError( "Internal Server Error"));
            }
        },
        Err(e) => {
            error!("Error getting user from id\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let mail = user.email.clone();

    let expiry: DateTime<Utc> = Utc::now().checked_add_days(Days::new(2)).unwrap();

    let link = match Link::create_from_user(db, user, expiry, LinkType::EmailChange).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating link\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let template = match load_template("email_change.html").await {
        Ok(a) => a.replace("${{CHANGE_URL}}", &link),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let message = match build_mail(&mail, "E-Mail Änderung", ContentType::TEXT_HTML, template) {
        Ok(a) => a,
        Err(e) => {
            error!("Error building message\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        return Err(error::ErrorInternalServerError( "Internal Server Error"));
    };

    Ok(web::Json(TextResponse { message: "Sent E-Mail, check your inbox".to_string()}))
}

