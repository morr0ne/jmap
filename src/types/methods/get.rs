use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Get<T> {
    pub state: String,
    pub list: Vec<T>,
    #[serde(rename = "notFound")]
    pub not_found: Vec<String>,
}

// TODO: requestTooLarge
