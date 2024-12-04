use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{error::Error, AppState};

pub type DBConnection = sqlx::PgPool;
pub type ConnectionData = actix_web::web::Data<crate::AppState>;



#[async_trait]
#[allow(clippy::upper_case_acronyms)]
pub trait CRUD<D, C>
where
    D: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + std::marker::Unpin,
    C: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
{
    async fn init_table(db: DBConnection) -> Result<(), Error>;

    async fn create(db: ConnectionData, data: C) -> Result<D, sqlx::Error>;

    async fn update_replace(db: ConnectionData, data: D) -> Result<D, sqlx::Error>;
    
    async fn create_id(db: ConnectionData, data: D) -> Result<D, sqlx::Error>;
    #[allow(dead_code)]
    async fn get_from_id(db: ConnectionData, id: (String, String)) -> Result<Option<D>, Error> {
        let res: Option<D> = sqlx::query_as("SELECT * FROM ? WHERE id = ?").bind(id.0).bind(id.1).fetch_optional(&db.db).await.expect("DB COnnection Failed");

        Ok(res)
    }

    async fn delete(db: ConnectionData, id: (String, String)) -> Result<(), Error> {
        sqlx::query("DELETE FROM ? WHERE id = ?").bind(id.0).bind(id.1).execute(&db.db).await.expect("DB Connection Failed");


        Ok(())
    }
}
