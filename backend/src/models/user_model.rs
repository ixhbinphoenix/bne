use std::collections::BTreeMap;

use actix_web::web::Data;
use backend_derive::{Creatable, Patchable};
use serde::{Serialize, Deserialize};
use surrealdb::{sql::{Value, Object, thing}, Response};

use crate::{utils::macros::map, database::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo}, prelude::*};

#[derive(Debug, Serialize, Deserialize, Creatable)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub person_id: i64,
    pub password_hash: String
}

impl From<User> for Value {
    fn from(value: User) -> Self {
        map![
            "id".into() => value.id.into(),
            "username".into() => value.username.into(),
            "person_id".into() => value.person_id.into(),
            "password_hash".into() => value.password_hash.into()
        ].into()
    }
}

impl TryFrom<Object> for User {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let id: String = W(match value.get("id") {
            Some(n) => n.to_owned(),
            None => return Err(Error::ConversionError("id".to_owned()))
        }).try_into()?;
        let username: String = W(match value.get("username") {
            Some(n) => n.to_owned(),
            None => return Err(Error::ConversionError("username".to_owned()))
        }).try_into()?;
        let person_id: i64 = W(match value.get("person_id") {
            Some(n) => n.to_owned(),
            None => return Err(Error::ConversionError("person_id".to_owned()))
        }).try_into()?;
        let password_hash: String = W(match value.get("password_hash") {
            Some(n) => n.to_owned(),
            None => return Err(Error::ConversionError("password_hash".to_owned()))
        }).try_into()?;

        Ok(User {
            id: Some(id),
            username,
            person_id,
            password_hash
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Patchable)]
pub struct UserPatch {
    pub username: Option<String>,
    pub person_id: Option<String>,
    pub password_hash: Option<String>
}

impl From<UserPatch> for Value {
    fn from(val: UserPatch) -> Self {
        let mut value: BTreeMap<String, Value> = BTreeMap::new();

        if let Some(t) = val.username {
            value.insert("username".into(), t.into());
        }
        Value::from(value)
    }
}

pub struct UserCRUD;

#[allow(dead_code)]
impl UserCRUD {
    pub async fn init_table(db: SurrealDBRepo) -> Result<Vec<Response>, Error> {
        let sql = "DEFINE TABLE users SCHEMAFULL;\
                   DEFINE FIELD username ON users TYPE string;\
                   DEFINE FIELD person_id ON users TYPE number;\
                   DEFINE INDEX person_id_index ON TABLE users COLUMNS person_id UNIQUE;\
                   DEFINE FIELD password_hash ON users TYPE string;";
        
        match db.ds.execute(sql, &db.ses, None, false).await {
            Ok(n) => Ok(n),
            Err(e) => {
                Err(Error::Surreal(e))
            },
        }
    }

    pub async fn create<T: Creatable>(db: Data<SurrealDBRepo>, data: T) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *;";

        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
            "tb".into() => "users".into(),
            "data".into() => data.into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_val = res.into_iter().next().map(|r| r.result).expect("id to be returned")?;

        W(first_val.first()).try_into()
    }

    pub async fn get_from_id(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th;";

        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("to get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_from_person_id(db: Data<SurrealDBRepo>, person_id: i64) -> Result<Object, Error> {
        let sql = "SELECT * FROM users WHERE personid=$personid";

        let vars: BTreeMap<String, Value> = map![
            "person_id".into() => person_id.to_string().into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = match res.into_iter().next() {
            Some(r) => r,
            None => {
                return Err(Error::ObjectNotFound(person_id.to_string()));
            }
        };

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_from_username(db: Data<SurrealDBRepo>, username: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM users WHERE username=$username;";

        let vars: BTreeMap<String, Value> = map![
            "username".into() => username.into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = match res.into_iter().next() {
            Some(r) => r,
            None => {
                return Err(Error::ObjectNotFound(username.to_owned()))
            },
        };

        W(first_res.result?.first()).try_into()
    }

    pub async fn update<T: Patchable>(db: Data<SurrealDBRepo>, tid: &str, data: T) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *;";

        let vars = map![
            "th".into() => thing(&tid)?.into(),
            "data".into() => data.into()
        ];
        
        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("id to be returned");

        let result = first_res.result?;

        W(result.first()).try_into()
    }

    pub async fn delete(db: Data<SurrealDBRepo>, tid: &str) -> Result<String, Error> {
        let sql = "DELETE $th RETURN *;";

        let vars = map!["th".into() => thing(&tid)?.into()];

        let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_res = res.into_iter().next().expect("id to be returned");

        first_res.result?;

        Ok(tid.to_string())
    }
}

