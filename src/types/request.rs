use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{Id, Invocation};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    using: Vec<String>,
    #[serde(rename = "methodCalls")]
    method_calls: Vec<Invocation>,
    #[serde(rename = "createdIds")]
    created_ids: Option<IndexMap<Id, Id>>,
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

    pub fn capability<C: Into<String>>(mut self, capability: C) -> Self {
        self.inner.using.push(capability.into());
        self
    }

    pub fn capabilities<C: IntoIterator<Item = String>>(mut self, capabilities: C) -> Self {
        self.inner.using.extend(capabilities);
        self
    }

    pub fn method<I: Into<Invocation>>(mut self, invocation: I) -> Self {
        self.inner.method_calls.push(invocation.into());
        self
    }

    pub fn methods<I: IntoIterator<Item = Invocation>>(mut self, invocations: I) -> Self {
        self.inner.method_calls.extend(invocations);
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
