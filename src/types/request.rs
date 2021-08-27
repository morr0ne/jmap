use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Invocation;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub using: Vec<String>,
    #[serde(rename = "methodCalls")]
    pub method_calls: Vec<Invocation>,
    #[serde(rename = "createdIds")]
    pub created_ids: Option<IndexMap<String, String>>, // TODO: parse ids
}

// TODO: Request-Level Errors https://www.rfc-editor.org/rfc/rfc8620.html#section-3.6.1
#[derive(Debug, Deserialize, Serialize)]
pub struct ResultReference {
    #[serde(rename = "resultOf")]
    pub result_of: String,
    pub name: String,
    pub path: String,
}
