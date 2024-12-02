use serde::{Deserialize, Serialize};

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Room {
    pub id: (String, String),
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, sqlxinsert::PgInsert)]
pub struct RoomCreate {
    pub name: String
}

#[async_trait::async_trait]
impl CRUD<Room, RoomCreate> for Room {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE rooms SCHEMAFULL;\
            DEFINE FIELD name ON rooms TYPE string;\
            DEFINE INDEX name ON rooms COLUMNS name UNIQUE;";
        db.query(sql).await?;
        Ok(())
    }
}

#[allow(dead_code)]
impl Room {
    pub async fn get_rooms(db: ConnectionData) -> Result<Vec<Room>, Error> {
        let mut res = db.query("SELECT * FROM rooms").await?;
        let rooms: Vec<Room> = res.take(0)?;

        Ok(rooms)
    }
}
