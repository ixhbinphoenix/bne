pub use crate::error::Error;

/// Macro that returns a 500 Internal Server Error JSON Response
/// Can take a string literal as an argument to change the error message
///
/// ## Usage
/// ```no_run
/// internalError!() // Returns a "Internal Server Error" with code 500
/// internalError!("Database Error") // Returns a "Database Error" with code 500
/// ```
#[macro_export]
macro_rules! internalError {
    ($l:literal) => {{
        return Ok(actix_web::web::Json($crate::api::response::Response::new_error(500, $l.to_string())));
    }};
    () => {{
        return Ok(actix_web::web::Json($crate::api::response::Response::new_error(
            500,
            "Internal Server Error".to_string(),
        )));
    }};
}
