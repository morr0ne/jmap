pub type AnyError = anyhow::Error;
pub type AnyResult<T> = anyhow::Result<T>;

pub type JsonMap<K, V> = serde_json::Map<K, V>;
pub type JsonValue = serde_json::Value;
pub type JsonObject = JsonMap<String, JsonValue>;

pub type HttpClient = reqwest::Client;

pub type Invocation<T = String, V = JsonValue> = (String, indexmap::IndexMap<T, V>, String); // TODO: parse invocation id

pub type Id = String;