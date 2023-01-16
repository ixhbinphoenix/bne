use std::sync::Arc;

use surrealdb::{sql::Value, Datastore, Session, Error};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        // TODO: File location or tikv, customizable
        let ds = Arc::new(Datastore::new("memory").await?);

        // TODO: Namespace and DB Customizable
        let ses = Session::for_kv().with_ns("test").with_db("test");

        Ok(SurrealDBRepo { ds, ses })
    }
}
