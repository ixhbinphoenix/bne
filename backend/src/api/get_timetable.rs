use actix_identity::Identity;
use actix_web::{error, web::{self}, HttpRequest, Responder, Result};
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{
    api_wrapper::{
        untis_client::UntisClient, utils::{FormattedLesson, TimetableParameter}
    }, models::{
        model::{DBConnection, CRUD}, user_model::User
    }, error::Error, utils::time::{format_for_untis, get_this_friday, get_this_monday}, GlobalUntisData
};


#[derive(Serialize)]
struct TimetableResponse {
    lessons: Vec<FormattedLesson>,
}

#[derive(Deserialize)]
pub struct TimetableQuery {
    from: Option<String>,
    until: Option<String>,
    class_name: Option<String>
}

pub async fn get_timetable(
    id: Option<Identity>, query: web::Query<TimetableQuery>, req: HttpRequest, untis_data: web::Data<GlobalUntisData>,
    db: web::Data<DBConnection>,
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden("Not logged in"));
    }

    let jsessionid = if let Some(session_cookie) = req.cookie("JSESSIONID") {
        session_cookie.value().to_string()
    } else {
        return Err(error::ErrorForbidden( "No JSESSIONID provided".to_string()));
    };

    let pot_user: Option<User> = User::get_from_id(
        db.clone(),
        match id.unwrap().id() {
            Ok(i) => {
                let split = i.split_once(':');
                if split.is_some() {
                    (split.unwrap().0.to_string(), split.unwrap().1.to_string())
                } else {
                    error!("ID in session_cookie is wrong???");
                    return Err(error::ErrorInternalServerError( "There was an error trying to get your id".to_string()));
                }
            }
            Err(e) => {
                error!("Error getting Identity id\n{e}");
                return Err(error::ErrorInternalServerError( "There was an error trying to get your id".to_string()));
            }
        },
    )
    .await?;

    let user = match pot_user {
        Some(u) => u,
        None => {
            debug!("Deleted(?) User tried to log in with old session token");
            return Err(error::ErrorNotFound( "This account doesn't exist!".to_string()));
        }
    };

    if !user.verified {
        return Err(error::ErrorUnauthorized("Account not verified! Check your E-Mails for a verification link".to_string()));
    }

    let untis = match UntisClient::unsafe_init(
        jsessionid,
        user.person_id.try_into().expect("the database to not store numbers bigger than u16"),
        5,
        "the-schedule".into(),
        untis_data.school.clone(),
        untis_data.subdomain.clone(),
        db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            if let Error::Reqwest(_) = e {
                return Err(error::ErrorUnprocessableEntity( "You done fucked up"));
            } else if let Error::UntisError(body) = e {
                return Err(error::ErrorInternalServerError( "Untis done fucked up ".to_string() + &body));
            }
            else {
                return Err(error::ErrorInternalServerError( "Some mysterious guy done fucked up"));
            }
        }
    };

    let from = match query.from.clone() {
        Some(from) => from,
        None => format_for_untis(get_this_monday()),
    };
    let until = match query.until.clone() {
        Some(until) => until,
        None => format_for_untis(get_this_friday()),
    };
    let class_name: Option<String> = query.class_name.clone();
    let timetable = match untis.clone().get_timetable(TimetableParameter::default(untis, from, until), class_name).await {
        Ok(timetable) => timetable,
        Err(err) => {
            return Err(error::ErrorInternalServerError( "Untis done fucked up ".to_string() + &err.to_string()));
        }
    };
    Ok(web::Json(TimetableResponse { lessons: timetable }))
}
