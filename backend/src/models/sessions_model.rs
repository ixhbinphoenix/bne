use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::model::ConnectionData;
use crate::error::Error;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    expiry: NaiveDateTime,
    id: (String, String),
    token: String,
}

#[allow(unused)]
impl Session {
    // This is very hacky but it works
    /// NEXER EXPOSE THIS FUNCTION TO USER INPUT, IT WILL ALLOW THEM TO SQL INJECT
    pub async fn delete_user_sessions(db: ConnectionData, id: String) -> Result<(), Error> {
        // Do not ever do this
        sqlx::query("DELETE sessions WHERE token = ?").bind(id).execute(&db.db).await.expect("DB Connection Failed");
        Ok(())
    }

    /// NEXER EXPOSE THIS FUNCTION TO USER INPUT, IT WILL ALLOW THEM TO SQL INJECT
    pub async fn get_user_sessions(db: ConnectionData, id: String) -> Result<Vec<Self>, Error> {
        // Once again, do not ever do this
        let res: Vec<Self> = sqlx::query_as("SELECT * FROM sessions WHERE token = ?").bind(id).fetch_all(&db.db).await.expect("DB Connection Failed");
        Ok(res)
    }
}
