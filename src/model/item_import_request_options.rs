use serde::{Serialize, Deserialize};
///An optional object to configure `/item/import` request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemImportRequestOptions {
    ///Specifies a webhook URL to associate with an Item. Plaid fires a webhook if credentials fail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for ItemImportRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}