use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jahrgang {
    pub id: Thing,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JahrgangCreate {
    pub name: String,
    pub active: bool,
}

#[async_trait::async_trait]
impl CRUD<Jahrgang, JahrgangCreate> for Jahrgang {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE jahrgaenge SCHEMAFULL;\
            DEFINE FIELD name ON jahrgaenge TYPE string;\
            DEFINE FIELD active ON jahrgaenge TYPE bool;\
            DEFINE INDEX name ON jahrgaenge COLUMNS name UNIQUE;";
        db.query(sql).await?;
        Ok(())
    }
}

#[allow(dead_code)]
impl Jahrgang {
    pub async fn get_jahrgaenge(db: ConnectionData) -> Result<Vec<Jahrgang>, Error> {
        let mut res = db.query("SELECT * FROM jahrgaenge").await?;
        let jahrgaenge: Vec<Jahrgang> = res.take(0)?;

        Ok(jahrgaenge)
    }

    pub async fn insert_one(db: ConnectionData, jahrgang: JahrgangCreate) -> Result<(), Error> {
        db.query("INSERT INTO jahrgaenge (name, active) VALUES ($name, $active)").bind(jahrgang).await?;
        Ok(())
    }

    pub async fn delete_all(db: ConnectionData) -> Result<(), Error> {
        db.query("DELETE FROM jahrgaenge;").await?;
        Ok(())
    }
}
