use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

use crate::types::common::JsonObject;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountCapabilities {
    #[serde(rename = "urn:ietf:params:jmap:core")]
    pub core: JsonObject,
    #[serde(rename = "urn:ietf:params:jmap:mail")]
    pub mail: Option<Mail>,
    #[serde(rename = "urn:ietf:params:jmap:submission")]
    pub submission: Option<Submission>,
    #[serde(rename = "urn:ietf:params:jmap:vacationresponse")]
    pub vacation_response: Option<JsonObject>,
    #[serde(flatten)]
    pub unknown: IndexMap<String, JsonObject>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Mail {
    pub maxMailboxesPerEmail: Option<NonZeroU32>,
    pub maxMailboxDepth: Option<u32>,
    pub maxSizeMailboxName: u32, // TODO: ensure it's at least 100
    pub maxSizeAttachmentsPerEmail: u32,
    pub emailQuerySortOptions: Vec<String>,
    pub mayCreateTopLevelMailbox: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Submission {
    pub maxDelayedSend: u32,
    // pub submissionExtensions: IndexMap<String, Vec<String>>, // TODO: cyrus server returns an array for some reason?
}
