use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::prelude::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Thing,
    pub email: String,
    pub person_id: i64,
    pub password_hash: String,
    pub untis_cypher: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCreate {
    pub email: String,
    pub person_id: i64,
    pub password_hash: String,
    pub untis_cypher: String,
}

#[async_trait::async_trait]
impl CRUD<User, UserCreate> for User {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE users SCHEMAFULL;\
                   DEFINE FIELD email ON users TYPE string ASSERT is::email($value);\
                   DEFINE INDEX email ON TABLE users COLUMNS email UNIQUE;\
                   DEFINE FIELD person_id ON users TYPE number;\
                   DEFINE INDEX person_id ON TABLE users COLUMNS person_id UNIQUE;\
                   DEFINE FIELD password_hash ON users TYPE string;\
                   DEFINE FIELD untis_cypher ON users TYPE string;";
        db.query(sql).await?;

        Ok(())
    }
}

#[allow(dead_code)]
impl User {
    pub async fn get_from_email(db: ConnectionData, email: String) -> Result<Option<User>, Error> {
        let mut res = db.query("SELECT * FROM users WHERE email=$email;").bind(("email", email)).await?;
        let user: Option<User> = res.take(0)?;

        Ok(user)
    }

    pub async fn get_from_person_id(db: ConnectionData, person_id: i64) -> Result<Option<User>, Error> {
        let mut res = db.query("SELECT * FROM users WHERE person_id=$id;").bind(("id", person_id)).await?;
        let user: Option<User> = res.take(0)?;

        Ok(user)
    }
}
