use serde::{Serialize, Deserialize};
use surrealdb::sql::{Datetime, Thing};

use crate::prelude::Error;

use super::model::ConnectionData;


#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    expiry: Datetime,
    id: Thing,
    token: String
}

#[allow(unused)]
impl Session {
    // This is very hacky but it works
    /// NEXER EXPOSE THIS FUNCTION TO USER INPUT, IT WILL ALLOW THEM TO SQL INJECT
    pub async fn delete_user_sessions(db: ConnectionData, id: String) -> Result<(), Error> {
        // Do not ever do this
        db.query(format!("DELETE sessions WHERE token = /.*{}.*/;", id)).await?;
        Ok(())
    }

    /// NEXER EXPOSE THIS FUNCTION TO USER INPUT, IT WILL ALLOW THEM TO SQL INJECT
    pub async fn get_user_sessions(db: ConnectionData, id: String) -> Result<Vec<Self>, Error> {
        // Once again, do not ever do this
        let res: Vec<Self> = db.query(format!("SELECT * FROM sessions WHERE token = /.*{}.*/", id)).await?.take(0)?;
        Ok(res)
    }
}
