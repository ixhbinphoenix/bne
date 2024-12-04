use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::error::Error;

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
        let res: Option<Vec<D>> = db.create(tb).content(data).await?;

        if let Some(mut res) = res {
            if !res.is_empty() {
                return Ok(res.pop().unwrap())
            }
        }
        Err(Error::DBOptionNone)
    }

    #[allow(dead_code)]
    async fn create_id(db: ConnectionData, id: (String, String), data: D) -> Result<D, Error> {
        let res: Option<D> = db.create(id).content(data).await?;

        match res {
            Some(a) => Ok(a),
            None => Err(Error::DBOptionNone)
        }
    }

    async fn get_from_id(db: ConnectionData, id: (String, String)) -> Result<Option<D>, Error> {
        let res: Option<D> = db.select(id).await?;

        Ok(res)
    }

    async fn update_replace(db: ConnectionData, id: (String, String), data: D) -> Result<(), Error> {
        let _: Option<D> = db.update(id).content(data).await?;

        Ok(())
    }

    async fn delete(db: ConnectionData, id: (String, String)) -> Result<(), Error> {
        let _: Option<D> = db.delete(id).await?;


        Ok(())
    }
}
