use actix_identity::Identity;
use actix_web::{Result, Responder, web};
use chrono::Local;
use lettre::{message::{header::ContentType, Mailbox, MultiPart, SinglePart, Attachment}, Message};
use serde::Serialize;
use surrealdb::sql::Thing;
use log::error;

use crate::{api::response::Response, internalError, models::{user_model::User, links_model::Link, model::{CRUD, ConnectionData}, sessions_model::Session}, mail::{utils::{load_template, Mailer}, mailing::send_mail}};

#[derive(Debug, Serialize)]
pub struct GDPRCompliance {
    user: User,
    links: Vec<Link>,
    sessions: Vec<Session>
}

pub async fn gdpr_data_compliance_get(id: Option<Identity>, db: ConnectionData, mailer: web::Data<Mailer>) -> Result<impl Responder> {
    if id.is_none() {
        return Ok(web::Json(Response::new_error(403, "Not logged in".into())));
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

    let links = match Link::get_from_user(db.clone(), user.clone()).await {
        Ok(a) => a,
        Err(e) => {
            error!("Error trying to get links\n{e}");
            internalError!()
        },
    };

    let sessions = match Session::get_user_sessions(db.clone(), user.clone().id.to_string()).await {
        Ok(a) => a,
        Err(e) => {
            error!("Error trying to get sessions\n{e}");
            internalError!()
        },
    };

    let compliance_data = GDPRCompliance {
        user: user.clone(),
        links,
        sessions
    };

    let mail = user.email.clone();

    let timestamp = Local::now().to_rfc2822();

    let template = match load_template("gdpr_compliance.html").await {
        Ok(a) => a.replace("${{TIMESTAMP}}", &timestamp),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Ok(Response::new_error(500, "Internal Server Error".into()).into());
        }
    };

    let address = match mail.parse::<Mailbox>() {
        Ok(a) => a,
        Err(e) => {
            error!("Error parsing mail\n{e}");
            internalError!()
        },
    };

    let json_data = match serde_json::to_string_pretty(&compliance_data) {
        Ok(a) => a,
        Err(e) => {
            error!("Error Serializing data to json\n{e}");
            internalError!()
        },
    };

    let message = Message::builder()
        .from("TheSchedule <noreply@theschedule.de>".parse().unwrap())
        .to(address)
        .subject("Deine Daten bei TheSchedule")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .content_type(ContentType::TEXT_HTML)
                        .body(template)
                )
                .singlepart(Attachment::new("daten.json".to_string()).body(
                        json_data,
                        ContentType::parse("application/json").unwrap()
                ))
        );

    let message = match message {
        Ok(a) => a,
        Err(e) => {
            error!("Error constructing message\n{e}");
            internalError!()
        },
    };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        internalError!()
    };

    Ok(web::Json(Response::new_success("Sent E-Mail, check your inbox".to_string())))
}
