use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Error {
    #[serde(rename = "statusCode")]
    pub status_code: String,
    pub message: String,
}
