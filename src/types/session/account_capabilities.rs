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
    #[serde(rename = "maxMailboxesPerEmail")]
    pub max_mailboxes_per_email: Option<NonZeroU32>,
    #[serde(rename = "maxMailboxDepth")]
    pub max_mailbox_depth: Option<u32>,
    #[serde(rename = "maxSizeMailboxName")]
    pub max_size_mailbox_name: u32, // TODO: ensure it's at least 100
    #[serde(rename = "maxSizeAttachmentsPerEmail")]
    pub max_size_attachments_per_email: u32,
    #[serde(rename = "emailQuerySortOptions")]
    pub email_query_sort_options: Vec<String>,
    #[serde(rename = "mayCreateTopLevelMailbox")]
    pub may_create_top_level_mailbox: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Submission {
    #[serde(rename = "maxDelayedSend")]
    pub max_delayed_send: u32,
    // pub submissionExtensions: IndexMap<String, Vec<String>>, // TODO: cyrus server returns an array for some reason?
}
