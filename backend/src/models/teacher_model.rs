use serde::{Deserialize, Serialize};

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Teacher {
    pub id: (String, String),
    pub shortname: String,
    pub longname: String,
    pub lessons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlxinsert::PgInsert)]
pub struct TeacherCreate {
    pub shortname: String,
    pub longname: String,
    pub lessons: Vec<String>,
}

#[async_trait::async_trait]
impl CRUD<Teacher, TeacherCreate> for Teacher {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE teachers SCHEMAFULL;\
            DEFINE FIELD shortname ON teachers TYPE string;\
            DEFINE INDEX shortname ON teachers COLUMNS shortname UNIQUE;\
            DEFINE FIELD longname ON teachers TYPE string;\
            DEFINE INDEX longname ON teachers COLUMNS longname UNIQUE;\
            DEFINE FIELD lessons ON teachers TYPE array;\
            DEFINE FIELD lessons.* ON teachers TYPE string;";
        sqlx::query(sql).execute(&db).await.expect("DB Connection Failed");

        Ok(())
    }
}

#[allow(dead_code)]
impl Teacher {
    pub async fn get_from_shortname(db: ConnectionData, shortname: String) -> Result<Option<Teacher>, Error> {
let res: Option<Teacher> = sqlx::query_as("SELECT * FROM teachers WHERE shortname=?;").bind(shortname).fetch_optional(&db.db).await.expect("DB Connection Failed");

        Ok(res)
    }

    pub async fn get_from_longname(db: ConnectionData, longname: String) -> Result<Option<Teacher>, Error> {
        let res: Option<Teacher> = sqlx::query_as("SELECT * FROM teachers WHERE longname=?;").bind(longname).fetch_optional(&db.db).await.expect("DB Connection Failed");

        Ok(res)
    }
}
