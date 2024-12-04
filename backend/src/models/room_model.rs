use serde::{Deserialize, Serialize};

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Room {
    pub id: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct RoomCreate {
    pub name: String
}

#[async_trait::async_trait]
impl CRUD<Room, RoomCreate> for Room {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE rooms SCHEMAFULL;\
            DEFINE FIELD name ON rooms TYPE string;\
            DEFINE INDEX name ON rooms COLUMNS name UNIQUE;";
        sqlx::query(sql).execute(&db).await.expect("DB Connection Failed");
        Ok(())
    }
    async fn create(db: ConnectionData, data: RoomCreate) -> Result<Room, sqlx::Error> {
        sqlx::query_as("INSERT INTO rooms (name) values (?);").bind(data.name).fetch_one(&db.db).await
    }
    async fn create_id(db: ConnectionData, data: Room) -> Result<Room, sqlx::Error> {
        sqlx::query_as("INSERT INTO rooms (name) values (?);").bind(data.name).fetch_one(&db.db).await
    }
    async fn update_replace(db: ConnectionData, data: Room) -> Result<Room, sqlx::Error> {
        sqlx::query_as("UPDATE rooms (name) values (?) WHERE id = ?;").bind(data.name).bind(data.id).fetch_one(&db.db).await
    }
}

#[allow(dead_code)]
impl Room {
    pub async fn get_rooms(db: ConnectionData) -> Result<Vec<Room>, Error> {
        let mut res: Vec<Room> = sqlx::query_as("SELECT * FROM rooms").fetch_all(&db.db).await.expect("DB Connection Failed");

        Ok(res)
    }
}
