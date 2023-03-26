use std::sync::Arc;

use surrealdb::{Datastore, Session, Error, sql::Value};

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session
}

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

impl SurrealDBRepo {
    pub async fn init(location: String, namespace: String, db: String) -> Result<Self, Error> {
        let ds = Arc::new(Datastore::new(&location).await?);

        let ses = Session::for_kv().with_ns(&namespace).with_db(&db);

        Ok(SurrealDBRepo { ds, ses })
    }
}
