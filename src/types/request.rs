use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{Invocation, JsonValue};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request<T = indexmap::IndexMap<String, JsonValue>> {
    pub using: Vec<String>,
    #[serde(rename = "methodCalls")]
    pub method_calls: Vec<Invocation<T>>,
    #[serde(rename = "createdIds")]
    pub created_ids: Option<IndexMap<String, String>>, // TODO: parse ids
}

pub struct RequestBuilder {
    capabilities: Vec<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    pub fn capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }

    pub fn build(self) -> Request {
        Request {
            using: self.capabilities,
            method_calls: Vec::new(),
            created_ids: None,
        }
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }
}

// TODO: Request-Level Errors https://www.rfc-editor.org/rfc/rfc8620.html#section-3.6.1
// #[derive(Debug, Deserialize, Serialize)]
// pub struct ResultReference {
//     #[serde(rename = "resultOf")]
//     pub result_of: String,
//     pub name: String,
//     pub path: String,
// }
