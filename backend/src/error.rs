// https://github.com/rust-awesome-app/template-app-base/blob/main/src-tauri/src/error.rs
// Licensed under Apache-2.0 and MIT

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use crate::{utils::password::PasswordError, mail::error::MailError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fail to get Ctx")]
    CtxFail,

    #[error("Fetching from Untis failed")]
    UntisError,

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    Password(#[from] PasswordError),

    #[error(transparent)]
    Mail(#[from] MailError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Password(_) => StatusCode::from_u16(403).expect("403 is an invalid status code now?"),
            _ => StatusCode::from_u16(500).expect("500 is an invalid status code now?"),
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let code = self.status_code();
        match code {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().body(format!("404 Not Found\n{self}")),
            StatusCode::FORBIDDEN => HttpResponse::Forbidden().body(format!("403 Forbidden\n{self}")),
            StatusCode::CONFLICT => HttpResponse::Conflict().body(format!("409 Conflict\n{self}")),
            StatusCode::INTERNAL_SERVER_ERROR => {
                HttpResponse::InternalServerError().body(format!("500 Internal Server Error\n{self}"))
            }
            // These 2⭥ are seperate if we ever wanna change the default/add a new code
            _ => HttpResponse::InternalServerError().body(format!("500 Internal Server Error\n{self}")),
        }
    }
}
