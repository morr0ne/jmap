use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Invocation;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "methodResponses")]
    pub method_responses: Vec<Invocation>,
    pub created_ids: Option<IndexMap<String, String>>, // TODO: parse ids
    #[serde(rename = "sessionState")]
    pub session_state: String,
}

// TODO: Method-Level Errors https://www.rfc-editor.org/rfc/rfc8620.html#section-3.6.2
