use serde::{Serialize, Deserialize};
///Conveys information about the errors causing missing or stale bank data used to construct the /signal/evaluate scores and response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalWarning {
    ///The warning code identifies a specific kind of warning that pertains to the error causing bank data to be missing. Safe for programmatic use. For more details on warning codes, please refer to Plaid standard error codes documentation. If you receive the `ITEM_LOGIN_REQUIRED` warning, we recommend re-authenticating your user by implementing Link's update mode. This will guide your user to fix their credentials, allowing Plaid to start fetching data again for future Signal requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<String>,
    ///A developer-friendly representation of the warning type. This may change over time and is not safe for programmatic use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_message: Option<String>,
    ///A broad categorization of the warning. Safe for programmatic use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_type: Option<String>,
}
impl std::fmt::Display for SignalWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}