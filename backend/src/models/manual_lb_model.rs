use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::model::{ConnectionData, DBConnection, CRUD};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ManualLB {
    pub id: String,
    pub teacher: String,
    pub room: String,
    pub start: i32,
    pub day: i32
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct ManualLBCreate {
    pub teacher: String,
    pub room: String,
    pub start: i32,
    pub day: i32
}

#[async_trait::async_trait]
impl CRUD<ManualLB, ManualLBCreate> for ManualLB {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE manual_lbs SCHEMAFULL;\
                   DEFINE FIELD teacher ON manual_lbs TYPE string;\
                   DEFINE FIELD room ON manual_lbs TYPE string;\
                   DEFINE FIELD start ON manual_lbs TYPE int;\
                   DEFINE FIELD day ON manual_lbs TYPE int;";
        sqlx::query(sql).execute(&db).await.expect("DB Connection Failed");

        Ok(())
    }
    async fn create(db: ConnectionData, data: ManualLBCreate) -> Result<ManualLB, sqlx::Error> {
        sqlx::query_as("INSERT INTO manual_lbs (teacher, room, start, day) values (?, ?, ?, ?);").bind(data.teacher).bind(data.room).bind(data.start).bind(data.day).fetch_one(&db.db).await
    }
    async fn create_id(db: ConnectionData, data: ManualLB) -> Result<ManualLB, sqlx::Error> {
        sqlx::query_as("INSERT INTO manual_lbs (id, teacher, room, start, day) values (?, ?, ?, ?, ?);").bind(data.id).bind(data.teacher).bind(data.room).bind(data.start).bind(data.day).fetch_one(&db.db).await
    }
    async fn update_replace(db: ConnectionData, data: ManualLB) -> Result<ManualLB, sqlx::Error> {
        sqlx::query_as("UPDATE manual_lbs (teacher, room, start, day) values (?, ?, ?, ?) WHERE id = ?;").bind(data.teacher).bind(data.room).bind(data.start).bind(data.day).bind(data.id).fetch_one(&db.db).await
    }
}

impl ManualLB {
    pub async fn get_manual_lbs(db: ConnectionData) -> Result<Vec<ManualLB>, Error> {
        let res = sqlx::query_as("SELECT * FROM manual_lbs").fetch_all(&db.db).await.expect("DB Connection Failed");

        Ok(res)
    }
}
