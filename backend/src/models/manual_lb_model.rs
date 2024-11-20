use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::error::Error;

use super::model::{ConnectionData, DBConnection, CRUD};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualLB {
    pub id: Thing,
    pub teacher: String,
    pub room: String,
    pub start: u8,
    pub day: u8
}

#[derive(Debug, Serialize, Deserialize)]
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
        let mut res = db.query("SELECT * FROM manual_lbs").await?;
        let lbs: Vec<ManualLB> = res.take(0)?;

        Ok(lbs)
    }
}
