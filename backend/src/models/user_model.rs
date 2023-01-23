use std::collections::BTreeMap;

use actix_web::web::Data;
use db_derive::{Creatable, Patchable};
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Value, Object, thing};

use crate::{utils::macros::map, database::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo}, prelude::*};

#[derive(Debug, Serialize, Deserialize, Creatable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String
}

impl From<User> for Value {
    fn from(value: User) -> Self {
        map![
            "id".into() => value.id.into(),
            "username".into() => value.username.into(),
            "password_hash".into() => value.password_hash.into()
        ].into()
    }
}

#[derive(Debug, Serialize, Deserialize, Patchable)]
pub struct UserPatch {
    pub username: Option<String>,
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

impl UserCRUD {
    pub async fn create<T: Creatable>(db: Data<SurrealDBRepo>, tb: &str, data: T) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *;";

        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
            "tb".into() => tb.into(),
            "data".into() => data.into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_val = res.into_iter().next().map(|r| r.result).expect("id to be returned")?;

        W(first_val.first()).try_into()
    }

    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th;";

        let tid = format!("messages:{}", tid);

        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("to get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn update<T: Patchable>(db: Data<SurrealDBRepo>, tid: &str, data: T) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *;";

        let tid = format!("messages:{}", tid);

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

        let tid = format!("messages:{}", tid);

        let vars = map!["th".into() => thing(&tid)?.into()];

        let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_res = res.into_iter().next().expect("id to be returned");

        first_res.result?;

        Ok(tid)
    }
}

