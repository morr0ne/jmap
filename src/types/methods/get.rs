use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(rename = "accountId")]
    pub account_id: Id,
    pub ids: Option<Vec<Id>>,
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub state: String,
    pub list: Vec<T>,
    #[serde(rename = "notFound")]
    pub not_found: Vec<String>,
}

// TODO: requestTooLarge
