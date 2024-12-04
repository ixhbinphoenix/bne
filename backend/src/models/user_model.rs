use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub person_id: i64,
    pub password_hash: String,
    pub untis_cypher: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, )]
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
        let sql = "CREATE TABLE IF NOT EXISTS users (
                   email VARCHAR UNIQUE, \
                   person_id int UNIQUE, \
                   password_hash VARCHAR, \
                   untis_cypher VARCHAR, \
                   verified BOOLEAN);";
        sqlx::query(sql).execute(&db).await.expect("Database Connection failed");
        Ok(())
    }
    async fn create(db: ConnectionData, data: UserCreate) -> Result<User, sqlx::Error> {
        sqlx::query_as("INSERT INTO users (email, person_id, password_hash, untis_cypher, verified) values (?, ?, ?, ?, ?);").bind(data.email).bind(data.person_id).bind(data.password_hash).bind(data.untis_cypher).bind(data.verified).fetch_one(&db.db).await
    }
    async fn create_id(db: ConnectionData, data: User) -> Result<User, sqlx::Error> {
        sqlx::query_as("INSERT INTO users (id, email, person_id, password_hash, untis_cypher, verified) values (?, ?, ?, ?, ?, ?);").bind(data.id).bind(data.email).bind(data.person_id).bind(data.password_hash).bind(data.untis_cypher).bind(data.verified).fetch_one(&db.db).await
    }
    async fn update_replace(db: ConnectionData, data: User) -> Result<User, sqlx::Error> {
        sqlx::query_as("UDPATE users (email, person_id, password_hash, untis_cypher, verified) values (?, ?, ?, ?, ?) WHERE id = ?;").bind(data.email).bind(data.person_id).bind(data.password_hash).bind(data.untis_cypher).bind(data.verified).bind(data.id).fetch_one(&db.db).await
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
