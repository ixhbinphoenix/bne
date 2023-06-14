use actix_identity::Identity;
use actix_web::{Result, Responder, web};
use chrono::{Utc, Months};
use lettre::message::header::ContentType;
use log::error;
use surrealdb::sql::Thing;

use crate::{models::{model::{CRUD, ConnectionData}, user_model::User, links_model::{Link, LinkType}}, internalError, mail::{utils::{load_template, Mailer}, mailing::{build_mail, send_mail}}};

use super::response::Response;


pub async fn resend_mail_get(db: ConnectionData, mailer: web::Data<Mailer>, id: Option<Identity>) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in".to_string())));
    }

    let id = id.unwrap();
    let id = match id.id() {
        Ok(a) => Thing::from(a.split_once(':').unwrap()),
        Err(e) => {
            error!("Error trying to get id\n{e}");
            internalError!()
        }
    };

    let user = match User::get_from_id(db.clone(), id.clone()).await {
        Ok(a) => match a {
            Some(a) => a,
            None => {
                error!("User not found?");
                internalError!()
            }
        },
        Err(e) => {
            error!("Error trying to get user\n{e}");
            internalError!()
        }
    };

    if user.clone().verified {
        return Ok(web::Json(Response::new_error(400, "You're already verified".to_string())));
    }

    if let Err(e) = Link::delete_from_user_type(db.clone(), user.clone(), LinkType::VerifyAccount).await {
        error!("Error deleting verification links\n{e}");
        internalError!()
    }

    let expiry_time = Utc::now().checked_add_months(Months::new(1)).unwrap();

    let link = match Link::create_from_user(db, user.clone(), expiry_time, LinkType::VerifyAccount).await {
        Ok(a) => a.construct_link(),
        Err(e) => {
            error!("Error creating link\n{e}");
            internalError!()
        }
    };

    let template = match load_template("verify.html").await {
        Ok(a) => a.replace("${{VERIFY_URL}}", &link),
        Err(e) => {
            error!("Error loading template\n{e}");
            internalError!()
        }
    };

    let message = match build_mail(&user.clone().email, "Accountverifizierung", ContentType::TEXT_HTML, template) {
        Ok(a) => a,
        Err(e) => {
            error!("Error building message\n{e}");
            internalError!()
        }
    };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        internalError!()
    }

    Ok(web::Json(Response::new_success("Sent E-Mail, check your inbox".to_string())))
}
