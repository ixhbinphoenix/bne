use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::prelude::Error;

pub type DBConnection = Surreal<Client>;
pub type ConnectionData = actix_web::web::Data<DBConnection>;

#[async_trait]
#[allow(clippy::upper_case_acronyms)]
pub trait CRUD<D, C>
where
    D: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
    C: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
{
    async fn init_table(db: DBConnection) -> Result<(), Error>;


    async fn create(db: ConnectionData, tb: String, data: C) -> Result<D, Error> {
        let res: D = db.create(tb).content(data).await?;

        Ok(res)
    }

    async fn create_id(db: ConnectionData, id: Thing, data: D) -> Result<D, Error> {
        let res: D = db.create(id).content(data).await?;

        Ok(res)
    }

    async fn get_from_id(db: ConnectionData, id: Thing) -> Result<Option<D>, Error> {
        let res: Option<D> = db.select(id).await?;

        Ok(res)
    }

    async fn update_replace(db: ConnectionData, id: Thing, data: D) -> Result<(), Error> {
        let _: D = db.update(id).content(data).await?;

        Ok(())
    }

    async fn delete(db: ConnectionData, id: Thing) -> Result<(), Error> {
        let _: D = db.delete(id).await?;


        Ok(())
    }
}
