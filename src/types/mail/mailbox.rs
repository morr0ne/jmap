use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Deserialize, Serialize)]

pub struct Mailbox {
    pub id: Id,
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub role: Option<String>, // TODO: parse roles https://www.iana.org/assignments/imap-mailbox-name-attributes/imap-mailbox-name-attributes.xhtml
    #[serde(default, rename = "sortOrder")]
    pub sort_order: u32,
    #[serde(rename = "totalEmails")]
    pub total_emails: u32,
    #[serde(rename = "unreadEmails")]
    pub unread_emails: u32,
    #[serde(rename = "totalThreads")]
    pub total_threads: u32,
    #[serde(rename = "unreadThreads")]
    pub unread_threads: u32,
    #[serde(rename = "myRights")]
    pub my_rights: MailboxRights,
    #[serde(rename = "isSubscribed")]
    pub is_subscribed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailboxRights {
    #[serde(rename = "mayReadItems")]
    pub may_read_items: bool,
    #[serde(rename = "mayAddItems")]
    pub may_add_items: bool,
    #[serde(rename = "mayRemoveItems")]
    pub may_remove_items: bool,
    #[serde(rename = "maySetSeen")]
    pub may_set_seen: bool,
    #[serde(rename = "maySetKeywords")]
    pub may_set_keywords: bool,
    #[serde(rename = "mayCreateChild")]
    pub may_create_child: bool,
    #[serde(rename = "mayRename")]
    pub may_rename: bool,
    #[serde(rename = "mayDelete")]
    pub may_delete: bool,
    #[serde(rename = "maySubmit")]
    pub may_submit: bool,
}
