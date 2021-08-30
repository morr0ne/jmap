use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{Id, Invocation, JsonValue};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request<T = indexmap::IndexMap<String, JsonValue>> {
    pub using: Vec<String>,
    #[serde(rename = "methodCalls")]
    pub method_calls: Vec<Invocation<T>>,
    #[serde(rename = "createdIds")]
    pub created_ids: Option<IndexMap<Id, Id>>,
}

pub struct RequestBuilder {
    inner: Request,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            inner: Request {
                using: Vec::new(),
                method_calls: Vec::new(),
                created_ids: None,
            },
        }
    }

    pub fn capability(mut self, capability: String) -> Self {
        self.inner.using.push(capability);
        self
    }

    pub fn build(self) -> Request {
        self.inner
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
