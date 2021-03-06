use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use url::Url;

mod account_capabilities;
mod capabilities;
pub use account_capabilities::AccountCapabilities;
pub use capabilities::Capabilities;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionObject {
    pub capabilities: Capabilities, // TODO: parse known capabilities
    pub accounts: IndexMap<String, Account>, // TODO: possibily parse id
    #[serde(rename = "primaryAccounts")]
    pub primary_accounts: IndexMap<String, String>, // TODO: possibily parse id
    pub username: String,
    #[serde(rename = "apiUrl")]
    pub api_url: Url,
    #[serde(rename = "downloadUrl")]
    pub download_url: Url,
    #[serde(rename = "uploadUrl")]
    pub upload_url: Url,
    #[serde(rename = "eventSourceUrl")]
    pub event_source_url: Url,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    #[serde(rename = "isPersonal")]
    pub is_personal: bool,
    #[serde(rename = "isReadOnly")]
    pub is_read_only: bool,
    #[serde(rename = "accountCapabilities")]
    pub account_capabilities: AccountCapabilities, // TODO: parse known capabilities
}
