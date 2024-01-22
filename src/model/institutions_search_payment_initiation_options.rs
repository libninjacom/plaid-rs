use serde::{Serialize, Deserialize};
///Additional options that will be used to filter institutions by various Payment Initiation configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    ///A unique ID identifying the payment consent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
    ///A unique ID identifying the payment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}
impl std::fmt::Display for InstitutionsSearchPaymentInitiationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}