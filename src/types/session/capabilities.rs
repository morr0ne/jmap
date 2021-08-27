use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::types::common::JsonObject;

#[derive(Debug, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "urn:ietf:params:jmap:core")]
    pub core: JsonObject,
    #[serde(flatten)]
    pub unknown: IndexMap<String, JsonObject>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Core {
    #[serde(rename = "maxSizeUpload")]
    pub max_size_upload: u32,
    #[serde(rename = "maxConcurrentUpload")]
    pub max_concurrent_upload: u32,
    #[serde(rename = "maxSizeRequest")]
    pub max_size_request: u32,
    #[serde(rename = "maxConcurrentRequests")]
    pub max_concurrent_requests: u32,
    #[serde(rename = "maxCallsInRequest")]
    pub max_calls_in_request: u32,
    #[serde(rename = "maxObjectsInGet")]
    pub max_objects_in_get: u32,
    #[serde(rename = "maxObjectsInSet")]
    pub max_objects_in_set: u32,
    #[serde(rename = "collationAlgorithms")]
    pub collation_algorithms: Vec<String>,
}
