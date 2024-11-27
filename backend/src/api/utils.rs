use serde::Serialize;

#[derive(Serialize)]
pub struct TextResponse {
    pub message: String
}