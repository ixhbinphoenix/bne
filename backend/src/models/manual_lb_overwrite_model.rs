use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::error::Error;

use super::model::{ConnectionData, DBConnection, CRUD};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualLBOverwrite {
    #[serde(skip_serializing)]
    pub id: Thing,
    pub teacher: String,
    pub start: u8,
    pub day: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManualLBOverwriteCreate {
    pub teacher: String,
    pub start: u8,
    pub day: u8,
}

#[async_trait::async_trait]
impl CRUD<ManualLBOverwrite, ManualLBOverwriteCreate> for ManualLBOverwrite {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE manual_lbs_overwrite SCHEMAFULL;\
                   DEFINE FIELD teacher ON manual_lbs_overwrite TYPE string;\
                   DEFINE FIELD start ON manual_lbs_overwrite TYPE int;\
                   DEFINE FIELD day ON manual_lbs_overwrite TYPE int;";
        db.query(sql).await?;

        Ok(())
    }
}

impl ManualLBOverwrite {
    pub async fn get_manual_lbs_overwrite(db: ConnectionData) -> Result<Vec<ManualLBOverwrite>, Error> {
        let mut res = db.query("SELECT * FROM manual_lbs_overwrite ORDER BY day, start").await?;
        let lbs: Vec<ManualLBOverwrite> = res.take(0)?;

        Ok(lbs)
    }
    pub async fn insert_one(db: ConnectionData, manuallb: ManualLBOverwriteCreate) -> Result<(), Error> {
        db.query("INSERT INTO manual_lbs_overwrite (teacher, day, start) VALUES ($teacher, $day, $start)")
            .bind(manuallb)
            .await?;
        Ok(())
    }
    pub async fn delete_all(db: ConnectionData) -> Result<(), Error> {
        db.query("DELETE FROM manual_lbs_overwrite;").await?;
        Ok(())
    }
}
