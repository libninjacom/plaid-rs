use serde::{Serialize, Deserialize};
///The reason for a failed transfer intent. Returned only if the transfer intent status is `failed`. Null otherwise.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferIntentGetFailureReason {
    ///A code representing the reason for a failed transfer intent (i.e., an API error or the authorization being declined).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    ///A human-readable description of the code associated with a failed transfer intent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    ///A broad categorization of the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}
impl std::fmt::Display for TransferIntentGetFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}