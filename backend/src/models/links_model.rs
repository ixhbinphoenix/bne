use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Thing, Id};
use uuid::Uuid;

use crate::prelude::Error;

use super::{model::{CRUD, DBConnection, ConnectionData}, user_model::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: Thing,
    pub user: Thing,
    pub link_type: LinkType,
    pub expiry: surrealdb::sql::Datetime
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LinkType {
    EmailReset,
    PasswordReset
}

pub type LinkCreate = Link;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkPatch {
    pub id: Thing,
    pub user: Option<Thing>,
    pub link_type: Option<Thing>
}

#[async_trait::async_trait]
impl CRUD<Link,LinkCreate,LinkPatch> for Link {
    async fn init_table(db: DBConnection) -> Result<bool, Error> {
        let sql = "DEFINE TABLE links SCHEMAFULL;\
                   DEFINE FIELD user ON links TYPE record(users);\
                   DEFINE FIELD link_type ON links TYPE string;";
        db.query(sql).await?;
        Ok(true)
    }
}

fn random_id() -> String {
    Uuid::new_v4().to_string()
}

#[allow(unused)]
impl Link {
    pub async fn create_from_user(db: ConnectionData, user: User, expiry: DateTime<Utc>, link_type: LinkType) -> Result<Self, Error> {
        let link_id = random_id();
        let user_id = user.id;
        let db_id = Thing { tb: "links".to_string(), id: Id::String(link_id)};

        let link = Link {
            id: db_id.clone(),
            user: user_id,
            link_type,
            expiry: expiry.into()
        };

        let res: Self = db.create(db_id)
            .content(link)
            .await?;

        Ok(res)
    }
}
