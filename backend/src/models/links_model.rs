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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LinkType {
    EmailChange,
    EmailReset,
    PasswordReset,
    VerifyAccount,
}

pub type LinkCreate = Link;

#[async_trait::async_trait]
impl CRUD<Link, LinkCreate> for Link {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE links SCHEMAFULL;\
                   DEFINE FIELD user ON links TYPE record(users);\
                   DEFINE FIELD link_type ON links TYPE string;\
                   DEFINE FIELD expiry ON links TYPE datetime;";
        db.query(sql).await?;
        Ok(())
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
                let _: Option<Link> = db.delete(id).await?;
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
        db: ConnectionData, user: User, expiry_time: DateTime<Utc>, link_type: LinkType,
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
            expiry: surrealdb::sql::Datetime::from(expiry_time),
        };

        let res: Option<Self> = db.create(db_id).content(link).await?;

        match res {
            Some(a) => Ok(a),
            None => Err(Error::DBOptionNone)
        }
    }

    pub async fn get_from_user(db: ConnectionData, user: User) -> Result<Vec<Self>, Error> {
        let sql = "SELECT * FROM links WHERE user=$user;";

        let res: Vec<Self> = db.query(sql).bind(("user", user.id)).await?.take(0)?;

        Ok(res)
    }

    pub async fn get_from_user_type(db: ConnectionData, user: User, link_type: LinkType) -> Result<Vec<Self>, Error> {
        let sql = "SELECT * FROM links WHERE user=$user AND link_type=$link_type;";

        let res: Vec<Self> = db.query(sql).bind(("user", user.id)).bind(("link_type", link_type)).await?.take(0)?;

        Ok(res)
    }

    pub async fn delete_from_user_type(db: ConnectionData, user: User, link_type: LinkType) -> Result<(), Error> {
        let sql = "DELETE links WHERE user=$user AND link_type=$link_type;";

        let res: Vec<Self> = db.query(sql).bind(("user", user.id)).bind(("link_type", link_type)).await?.take(0)?;

        Ok(())
    }

    pub fn construct_link(&self) -> String {
        let typestr = match self.link_type {
            LinkType::EmailChange => "change-email",
            LinkType::EmailReset => "reset-email",
            LinkType::PasswordReset => "reset-password",
            LinkType::VerifyAccount => "verify",
        };
        if cfg!(debug_assertions) {
            format!("http://localhost:3000/{}/{}", typestr, self.id.id.to_raw())
        } else {
            format!("https://theschedule.de/{}/{}", typestr, self.id.id.to_raw())
        }
    }
}
