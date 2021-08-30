use serde::{Deserialize, Serialize};

use super::JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct Invocation<T = indexmap::IndexMap<String, JsonValue>>(pub String, pub T, pub String);
