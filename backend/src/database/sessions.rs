use crate::{models::model::ConnectionData, prelude::Error};

// This is very hacky but it works
pub async fn delete_user_sessions(db: ConnectionData, id: String) -> Result<(), Error> {
    // Do not ever do this
    db.query(format!("DELETE sessions WHERE token = /.*{}.*/;", id)).await?;
    Ok(())
}
