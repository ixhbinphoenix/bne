use crate::{models::model::ConnectionData, error::Error};

// This is very hacky but it works
/// NEXER EXPOSE THIS FUNCTION TO USER INPUT, IT WILL ALLOW THEM TO SQL INJECT
pub async fn delete_user_sessions(db: ConnectionData, id: String) -> Result<(), Error> {
    // Do not ever do this
    
    Ok(())
}
