use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Mailbox {
    pub id: String, // TODO: parse id
    pub name: String,
    pub parentId: Option<String>,
    pub role: Option<String>, // TODO: parse roles https://www.iana.org/assignments/imap-mailbox-name-attributes/imap-mailbox-name-attributes.xhtml
    #[serde(default)]
    pub sortOrder: u32,
    pub totalEmails: u32,
    pub unreadEmails: u32,
    pub totalThreads: u32,
    pub unreadThreads: u32,
    pub myRights: MailboxRights,
    pub isSubscribed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailboxRights {
    pub mayReadItems: bool,
    pub mayAddItems: bool,
    pub mayRemoveItems: bool,
    pub maySetSeen: bool,
    pub maySetKeywords: bool,
    pub mayCreateChild: bool,
    pub mayRename: bool,
    pub mayDelete: bool,
    pub maySubmit: bool,
}
