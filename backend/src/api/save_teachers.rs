use actix_identity::Identity;
use actix_web::{error, web, Responder, Result};
use log::{debug, error};
use surrealdb::sql::Thing;

use crate::models::{model::{DBConnection, CRUD}, teacher_model::{Teacher, TeacherCreate}, user_model::User};

use super::utils::TextResponse;


pub async fn save_teachers(
    data: web::Json<Vec<TeacherCreate>>,
    id: Option<Identity>,
    db: web::Data<DBConnection>
) -> Result<impl Responder> {
    if id.is_none() {
        return Err(error::ErrorForbidden( "Not logged in".to_string()));
    }
    let pot_user: Option<User> = User::get_from_id(
        db.clone(),
        match id.unwrap().id() {
            Ok(i) => {
                let split = i.split_once(':');
                if split.is_some() {
                    Thing::from(split.unwrap())
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
        return Err(error::ErrorUnauthorized("Account not verified! Check your E-Mails for a verification link"));
    }

    Teacher::delete_all(db.clone()).await?;

    for teacher in data.clone().into_iter() {
        Teacher::insert_one(db.clone(), teacher).await?
    }


    Ok(web::Json(TextResponse {message: "Teachers updated".to_string()}))
}