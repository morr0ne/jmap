use serde::{Deserialize, Serialize};

use super::JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct Invocation<T = JsonValue>(pub String, pub indexmap::IndexMap<String, T>, pub String);
