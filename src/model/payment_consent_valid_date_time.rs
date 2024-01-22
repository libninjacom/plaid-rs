use serde::{Serialize, Deserialize};
///Life span for the payment consent. After the `to` date the payment consent expires and can no longer be used for payment initiation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentConsentValidDateTime {
    ///The date and time from which the consent should be active, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    ///The date and time at which the consent expires, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for PaymentConsentValidDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}