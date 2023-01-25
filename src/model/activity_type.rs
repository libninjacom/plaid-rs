
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ITEM_CREATE")]
    ItemCreate,
    #[serde(rename = "ITEM_IMPORT")]
    ItemImport,
    #[serde(rename = "ITEM_UPDATE")]
    ItemUpdate,
    #[serde(rename = "ITEM_UNLINK")]
    ItemUnlink,
    #[serde(rename = "PORTAL_UNLINK")]
    PortalUnlink,
    #[serde(rename = "PORTAL_ITEMS_DELETE")]
    PortalItemsDelete,
    #[serde(rename = "ITEM_REMOVE")]
    ItemRemove,
    #[serde(rename = "INVARIANT_CHECKER_DELETION")]
    InvariantCheckerDeletion,
    #[serde(rename = "SCOPES_UPDATE")]
    ScopesUpdate,
}