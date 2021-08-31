use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(rename = "accountId")]
    pub account_id: Id,
    #[serde(rename = "maxChanges")]
    pub max_changes: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "accountId")]
    pub account_id: Id,
    #[serde(rename = "oldState")]
    pub old_state: String,
    #[serde(rename = "newState")]
    pub new_state: String,
    #[serde(rename = "hasMoreChanges")]
    pub has_more_changes: bool,
    pub created: Vec<Id>,
    pub updated: Vec<Id>,
    pub destroyed: Vec<Id>,
}
