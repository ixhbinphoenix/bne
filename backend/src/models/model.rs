use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::prelude::Error;

pub type DBConnection = Surreal<Client>;
pub type ConnectionData = actix_web::web::Data<DBConnection>;

#[async_trait]
#[allow(clippy::upper_case_acronyms)]
pub trait CRUD<D, C, P>
where
    D: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
    C: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
    P: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
{
    async fn init_table(db: DBConnection) -> Result<bool, Error>;

    async fn create(db: ConnectionData, tb: String, data: C) -> Result<D, Error> {
        let res: D = db.create(tb).content(data).await?;

        Ok(res)
    }

    async fn get_from_id(db: ConnectionData, id: Thing) -> Result<Option<D>, Error> {
        let res: Option<D> = db.select(id).await?;

        Ok(res)
    }

    async fn update_replace(db: ConnectionData, id: Thing, data: C) -> Result<bool, Error> {
        db.update(id).merge(data).await?;

        Ok(true)
    }

    async fn update_merge(db: ConnectionData, id: Thing, data: P) -> Result<bool, Error> {
        db.update(id).merge(data).await?;

        Ok(true)
    }

    async fn delete(db: ConnectionData, id: Thing) -> Result<(), Error> {
        db.delete(id).await?;

        Ok(())
    }
}