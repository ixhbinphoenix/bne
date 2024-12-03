use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: (String, String),
    pub email: String,
    pub person_id: i64,
    pub password_hash: String,
    pub untis_cypher: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlxinsert::PgInsert)]
pub struct UserCreate {
    pub email: String,
    pub person_id: i64,
    pub password_hash: String,
    pub untis_cypher: String,
    pub verified: bool,
}

#[async_trait::async_trait]
impl CRUD<User, UserCreate> for User {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE users SCHEMAFULL;\
                   DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);\
                   DEFINE INDEX email ON TABLE users COLUMNS email UNIQUE;\
                   DEFINE FIELD person_id ON users TYPE number;\
                   DEFINE INDEX person_id ON TABLE users COLUMNS person_id UNIQUE;\
                   DEFINE FIELD password_hash ON users TYPE string;\
                   DEFINE FIELD untis_cypher ON users TYPE string;\
                   DEFINE FIELD verified ON users TYPE bool;";
        sqlx::query(sql).execute(&db).await.expect("Database Connection failed");
        Ok(())
    }
}

#[allow(dead_code)]
impl User {
    pub async fn get_from_email(db: ConnectionData, email: String) -> Result<Option<User>, Error> {
        let res: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = ?").bind(email).fetch_optional(&db.db).await.expect("DB Connection Failed");
        Ok(res)
    }

    pub async fn get_from_person_id(db: ConnectionData, person_id: i64) -> Result<Option<User>, Error> {
        let res: Option<User> = sqlx::query_as("SELECT * FROM users WHERE person_id=?;").bind(person_id).fetch_optional(&db.db).await.expect("DB Connection Failed");
        Ok(res)
    }

    pub fn verify_password(&self, password: String) -> Result<(), argon2::password_hash::Error> {
        let argon2 = Argon2::default();

        let hash = PasswordHash::new(&self.password_hash)?;

        argon2.verify_password(password.as_bytes(), &hash)
    }
}
