use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use super::{
    model::{ConnectionData, DBConnection, CRUD}, user_model::User
};
use crate::{prelude::Error, utils::uuid::random_id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: Thing,
    pub user: Thing,
    pub link_type: LinkType,
    pub expiry: surrealdb::sql::Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LinkType {
    EmailReset,
    PasswordReset,
    VerifyAccount,
}

pub type LinkCreate = Link;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkPatch {
    pub id: Thing,
    pub user: Option<Thing>,
    pub link_type: Option<Thing>,
}

#[async_trait::async_trait]
impl CRUD<Link, LinkCreate, LinkPatch> for Link {
    async fn init_table(db: DBConnection) -> Result<bool, Error> {
        let sql = "DEFINE TABLE links SCHEMAFULL;\
                   DEFINE FIELD user ON links TYPE record(users);\
                   DEFINE FIELD link_type ON links TYPE string;";
        db.query(sql).await?;
        Ok(true)
    }

    /// DO NOT USE THIS IT WILL ALWAYS ERROR
    /// USE `Link::create_from_user` OR `Link::create_id` WITH A RANDOM UUIDv4 INSTEAD
    async fn create(_: ConnectionData, _: String, _: LinkCreate) -> Result<Link, Error> {
        panic!("I fucking warned you dude. I told you bro. (https://cat-girls.club/k2d0WWDA)")
    }

    async fn get_from_id(db: ConnectionData, id: Thing) -> Result<Option<Link>, Error> {
        let res: Option<Link> = db.select(id.clone()).await?;

        if let Some(link) = res {
            if link.expiry.timestamp_millis() < Utc::now().timestamp_millis() {
                debug!("Link expired, deleting.");
                db.delete(id).await?;
                Ok(None)
            } else {
                Ok(Some(link))
            }
        } else {
            Ok(None)
        }
    }
}

#[allow(unused)]
impl Link {
    pub async fn create_from_user(
        db: ConnectionData, user: User, expiry: DateTime<Utc>, link_type: LinkType,
    ) -> Result<Self, Error> {
        let link_id = random_id();
        let user_id = user.id;
        let db_id = Thing {
            tb: "links".to_string(),
            id: Id::String(link_id),
        };

        let link = Link {
            id: db_id.clone(),
            user: user_id,
            link_type,
            expiry: expiry.into(),
        };

        let res: Self = db.create(db_id).content(link).await?;

        Ok(res)
    }

    pub fn construct_link(&self) -> String {
        let typestr = match self.link_type {
            LinkType::EmailReset => "reset-email",
            LinkType::PasswordReset => "reset-password",
            LinkType::VerifyAccount => "verify",
        };
        format!("https://theschedule.de/{}/{}", typestr, self.id.id.to_string())
    }
}
