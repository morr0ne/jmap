use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(rename = "accountId")]
    pub account_id: Id,
    pub maxChanges: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "accountId")]
    pub account_id: Id,
    pub oldState: String,
    pub newState: String,
    pub hasMoreChanges: bool,
    pub created: Vec<Id>,
    pub updated: Vec<Id>,
    pub destroyed: Vec<Id>,
}
