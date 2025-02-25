use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    #[serde(skip_serializing)]
    pub _id: Thing,
    pub shortname: String,
    pub longname: String,
    pub lessons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        db.query(sql).await?;

        Ok(())
    }
}

#[allow(dead_code)]
impl Teacher {
    pub async fn get_from_shortname(db: ConnectionData, shortname: String) -> Result<Option<Teacher>, Error> {
        let mut res = db.query("SELECT * FROM teachers WHERE shortname=$name;").bind(("name", shortname)).await?;
        let teacher: Option<Teacher> = res.take(0)?;

        Ok(teacher)
    }

    pub async fn get_from_longname(db: ConnectionData, longname: String) -> Result<Option<Teacher>, Error> {
        let mut res = db.query("SELECT * FROM teachers WHERE longname=$name;").bind(("name", longname)).await?;
        let teacher: Option<Teacher> = res.take(0)?;

        Ok(teacher)
    }
    pub async fn get_all(db: ConnectionData) -> Result<Vec<Teacher>, Error> {
        let mut res = db.query("SELECT * FROM teachers ORDER BY shortname;").await?;
        let teachers = res.take(0)?;
        Ok(teachers)
    }
    pub async fn insert_one(db: ConnectionData, teacher: TeacherCreate) -> Result<(), Error> {
        db.query("INSERT INTO teachers (longname, shortname, lessons) VALUES ($longname, $shortname, $lessons)").bind(teacher).await?;
        Ok(())
    }
    pub async fn delete_all(db: ConnectionData) -> Result<(), Error> {
        db.query("DELETE FROM teachers;").await?;
        Ok(())
    }
}
