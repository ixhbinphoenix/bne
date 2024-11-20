use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use chrono::{Months, Utc};
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


pub async fn resend_mail_get(
    db: ConnectionData, mailer: web::Data<Mailer>, id: Option<Identity>,
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden( "Not logged in".to_string()));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => Thing::from(a.split_once(':').unwrap()),
        Err(e) => {
            error!("Error trying to get id\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let user = match User::get_from_id(db.clone(), id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                return Err(error::ErrorInternalServerError("Internal Server Error"));
            }
        },
        Err(e) => {
            error!("Error trying to get user\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    if user.clone().verified {
        return Err(error::ErrorUnprocessableEntity( "You're already verified".to_string()));
    }

    if let Err(e) = Link::delete_from_user_type(db.clone(), user.clone(), LinkType::VerifyAccount).await {
        error!("Error deleting verification links\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    }

    let expiry_time = Utc::now().checked_add_months(Months::new(1)).unwrap();

    let link = match Link::create_from_user(db, user.clone(), expiry_time, LinkType::VerifyAccount).await {
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

    let message = match build_mail(&user.clone().email, "Accountverifizierung", ContentType::TEXT_HTML, template) {
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

    Ok(web::Json(TextResponse { message: "Sent E-Mail, check your inbox".to_string()}))
}
