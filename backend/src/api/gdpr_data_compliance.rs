use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use chrono::Local;
use lettre::{
    message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart}, Message
};
use log::error;
use serde::Serialize;

use crate::{
    api::utils::TextResponse, mail::{
        mailing::send_mail, utils::{load_template, Mailer}
    }, models::{
        links_model::Link, model::{ConnectionData, CRUD}, sessions_model::Session, user_model::User
    }
};

#[derive(Debug, Serialize)]
pub struct GDPRCompliance {
    user: User,
    links: Vec<Link>,
    sessions: Vec<Session>,
}

pub async fn gdpr_data_compliance_get(
    id: Option<Identity>, db: ConnectionData, mailer: web::Data<Mailer>,
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

    let links = match Link::get_from_user(db.clone(), user.clone()).await {
        Ok(a) => a,
        Err(e) => {
            error!("Error trying to get links\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let user_id = user.clone().id;
    let sessions = match Session::get_user_sessions(db.clone(), format!("{}:{}", user_id.0, user_id.1)).await {
        Ok(a) => a,
        Err(e) => {
            error!("Error trying to get sessions\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let compliance_data = GDPRCompliance {
        user: user.clone(),
        links,
        sessions,
    };

    let mail = user.email.clone();

    let timestamp = Local::now().to_rfc2822();

    let template = match load_template("gdpr_compliance.html").await {
        Ok(a) => a.replace("${{TIMESTAMP}}", &timestamp),
        Err(e) => {
            error!("Error loading template\n{e}");
            return Err(error::ErrorInternalServerError( "Internal Server Error"));
        }
    };

    let address = match mail.parse::<Mailbox>() {
        Ok(a) => a,
        Err(e) => {
            error!("Error parsing mail\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let json_data = match serde_json::to_string_pretty(&compliance_data) {
        Ok(a) => a,
        Err(e) => {
            error!("Error Serializing data to json\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    let message = Message::builder()
        .from("TheSchedule <noreply@theschedule.de>".parse().unwrap())
        .to(address)
        .subject("Deine Daten bei TheSchedule")
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::builder().content_type(ContentType::TEXT_HTML).body(template))
                .singlepart(
                    Attachment::new("daten.json".to_string())
                        .body(json_data, ContentType::parse("application/json").unwrap()),
                ),
        );

    let message = match message {
        Ok(a) => a,
        Err(e) => {
            error!("Error constructing message\n{e}");
            return Err(error::ErrorInternalServerError("Internal Server Error"));
        }
    };

    if let Err(e) = send_mail(mailer, message).await {
        error!("Error sending mail\n{e}");
        return Err(error::ErrorInternalServerError("Internal Server Error"));
    };

    Ok(web::Json(TextResponse { message: "Sent E-Mail, check your inbox".to_string()}))
}
