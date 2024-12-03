use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::model::{ConnectionData, DBConnection, CRUD};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ManualLB {
    pub id: (String, String),
    pub teacher: String,
    pub room: String,
    pub start: i32,
    pub day: i32
}

#[derive(Debug, Serialize, Deserialize, sqlxinsert::PgInsert)]
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
