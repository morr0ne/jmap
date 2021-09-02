use super::{JsonMap, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Invocation(String, JsonMap<String, JsonValue>, String);

impl Invocation {
    pub fn new<N: Into<String>, A: IntoIterator<Item = (String, JsonValue)>, I: Into<String>>(
        name: N,
        arguments: A,
        id: I,
    ) -> Self {
        Self(name.into(), arguments.into_iter().collect(), id.into())
    }
}
