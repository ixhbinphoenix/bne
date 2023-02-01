use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Parameter{
    AuthParameter(AuthParameter),
    DateParameter(DateParameter),
    TimeTableParameter(TimeTableParameter),
    Null()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UntisBody {
    pub school: String,
    pub id: String,
    pub method: String,
    pub params: Parameter,
    pub jsonrpc: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthParameter {
    pub user: String,
    pub password: String,
    pub client: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateParameter {
    pub options: Options
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableParameter {
    pub id: String,
    pub r#type: String,
    pub start_date: String,
    pub end_date: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub start_date: String,
    pub end_date: String
}
