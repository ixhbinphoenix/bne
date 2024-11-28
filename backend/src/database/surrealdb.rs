use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: (String, String),
}
