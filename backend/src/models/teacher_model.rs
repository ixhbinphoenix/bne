use serde::{Deserialize, Serialize};

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Teacher {
    pub id: String,
    pub shortname: String,
    pub longname: String,
    pub lessons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct TeacherCreate {
    pub shortname: String,
    pub longname: String,
    pub lessons: Vec<String>,
}

#[async_trait::async_trait]
impl CRUD<Teacher, TeacherCreate> for Teacher {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "CREATE TABLE IF NOT EXISTS teachers (
                        shortname VARCHAR,
                        longname VARCHAR,
                        lessons TEXT[]
                        );

                        CREATE UNIQUE INDEX shortname_idx ON teachers (shortname);
                        CREATE UNIQUE INDEX longname_idx ON teachers (longname);";
        sqlx::query(sql).execute(&db).await.expect("DB Connection Failed");

        Ok(())
    }
    async fn create(db: ConnectionData, data: TeacherCreate) -> Result<Teacher, sqlx::Error> {
        sqlx::query_as("INSERT INTO teachers (shortname, longname, lessons) values (?, ?, ?);").bind(data.shortname).bind(data.longname).bind(data.lessons).fetch_one(&db.db).await
    }
    async fn create_id(db: ConnectionData, data: Teacher) -> Result<Teacher, sqlx::Error> {
        sqlx::query_as("INSERT INTO teachers (shortname, longname, lessons) values (?, ?, ?);").bind(data.shortname).bind(data.longname).bind(data.lessons).fetch_one(&db.db).await
    }
    async fn update_replace(db: ConnectionData, data: Teacher) -> Result<Teacher, sqlx::Error> {
        sqlx::query_as("UPDATE teachers (shortname, longname, lessons) values (?, ?, ?) WHERE id = ?;").bind(data.shortname).bind(data.longname).bind(data.lessons).bind(data.id).fetch_one(&db.db).await
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
