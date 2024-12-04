use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, types::chrono};

use super::{
    model::{ConnectionData, DBConnection, CRUD}, user_model::User
};
use crate::{error::Error, utils::uuid::random_id};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Link {
    pub id: String,
    pub user: String,
    pub link_type: LinkType,
    pub expiry: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all="kebab-case")]
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
                   DEFINE FIELD user ON links TYPE record<users>;\
                   DEFINE FIELD link_type ON links TYPE string;\
                   DEFINE FIELD expiry ON links TYPE datetime;";
        sqlx::query(sql).execute(&db).await.expect("DB Connection Failed");
        Ok(())
    }

    /// DO NOT USE THIS IT WILL ALWAYS ERROR
    /// USE `Link::create_from_user` OR `Link::create_id` WITH A RANDOM UUIDv4 INSTEAD
    async fn create(_: ConnectionData, _: LinkCreate) -> Result<Link, sqlx::Error> {
        panic!("I fucking warned you dude. I told you bro. (https://cat-girls.club/k2d0WWDA)")
    }
    async fn create_id(db: ConnectionData, data: Link) -> Result<Link, sqlx::Error> {
        sqlx::query_as("INSERT INTO links (id, user, link_type, expiry) values (?, ?, ?, ?);").bind(data.id).bind(data.user).bind(data.link_type).bind(data.expiry).fetch_one(&db.db).await
    }
    async fn update_replace(db: ConnectionData, data: Link) -> Result<Link, sqlx::Error> {
        unimplemented!()
    }
    async fn get_from_id(db: ConnectionData, id: (String, String)) -> Result<Option<Link>, Error> {
        let res: Option<Link> = sqlx::query_as("SELECT id, user, link_type AS \"link_type: LinkType\", expiry FROM links WHERE user=?;").bind(id.1.clone()).fetch_optional(&db.db).await.expect("DB Connection Failed");

        if let Some(link) = res {
            if link.expiry.and_utc().timestamp_millis() < Utc::now().timestamp_millis() {
                debug!("Link expired, deleting.");
                let _: PgQueryResult
                 = sqlx::query("DELETE FROM links WHERE id = ?").bind(id.1).execute(&db.db).await.expect("DB Connection Failed");
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
    ) -> Result<Link, Error> {
        let link_id = random_id();

        let res: Link = sqlx::query_as("INSERT INTO links (id , user, expiry, link_type) values (?, ?, ?, ?) RETURNING id, user, link_type AS \"link_type: LinkType\", expiry;").bind(link_id).bind(user.id).bind(expiry_time).bind(serde_json::to_string(&link_type).unwrap()).fetch_one(&db.db).await.expect("DB Connection Failed");

        Ok(res)
    }

    pub async fn get_from_user(db: ConnectionData, user: User) -> Result<Vec<Link>, sqlx::Error> {
        sqlx::query_as("SELECT id, user, link_type AS \"link_type: LinkType\", expiry FROM links WHERE user=?;").bind(user.id).fetch_all(&db.db).await
    }

    pub async fn get_from_user_type(db: ConnectionData, user: User, link_type: LinkType) -> Result<Vec<Self>, Error> {

        let res: Vec<Self> = sqlx::query_as("SELECT * FROM links WHERE user=? AND link_type=?;").bind(user.id).bind(link_type).fetch_all(&db.db).await.expect("DB Connection Failed");

        Ok(res)
    }

    pub async fn delete_from_user_type(db: ConnectionData, user: User, link_type: LinkType) -> Result<(), Error> {
        let res: PgQueryResult = sqlx::query("DELETE links WHERE user=? AND link_type=?").bind(user.id).bind(link_type).execute(&db.db).await.expect("DB Connection Failed");

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
            format!("http://localhost:3000/{}/{}", typestr, self.id)
        } else {
            format!("https://theschedule.de/{}/{}", typestr, self.id)
        }
    }
}
