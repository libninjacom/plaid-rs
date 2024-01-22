use serde::{Serialize, Deserialize};
/**A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.

Example: `{"GBP": "10000"}`*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationMaximumPaymentAmount {}
impl std::fmt::Display for PaymentInitiationMaximumPaymentAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}