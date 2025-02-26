use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::error::Error;

use super::model::{ConnectionData, DBConnection, CRUD};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualLB {
    #[serde(skip_serializing)]
    pub id: Thing,
    pub teacher: String,
    pub room: String,
    pub start: u8,
    pub day: u8
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManualLBCreate {
    pub teacher: String,
    pub room: String,
    pub start: u8,
    pub day: u8
}

#[async_trait::async_trait]
impl CRUD<ManualLB, ManualLBCreate> for ManualLB {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE manual_lbs SCHEMAFULL;\
                   DEFINE FIELD teacher ON manual_lbs TYPE string;\
                   DEFINE FIELD room ON manual_lbs TYPE string;\
                   DEFINE FIELD start ON manual_lbs TYPE int;\
                   DEFINE FIELD day ON manual_lbs TYPE int;";
        db.query(sql).await?;

        Ok(())
    }
}

impl ManualLB {
    pub async fn get_manual_lbs(db: ConnectionData) -> Result<Vec<ManualLB>, Error> {
        let mut res = db.query("SELECT * FROM manual_lbs ORDER BY day, start").await?;
        let lbs: Vec<ManualLB> = res.take(0)?;

        Ok(lbs)
    }
    pub async fn insert_one(db: ConnectionData, manuallb: ManualLBCreate) -> Result<(), Error> {
        db.query("INSERT INTO manual_lbs (teacher, day, start, room) VALUES ($teacher, $day, $start, $room)").bind(manuallb).await?;
        Ok(())
    }
    pub async fn delete_all(db: ConnectionData) -> Result<(), Error> {
        db.query("DELETE FROM manual_lbs;").await?;
        Ok(())
    }
}
