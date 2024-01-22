use serde::{Serialize, Deserialize};
///Information about the accounts that the payment was distributed to.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistributionBreakdown {
    ///Name of the account for the given distribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    ///The name of the bank that the payment is being deposited to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///The amount distributed to this account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///The last 2-4 alphanumeric characters of an account's official account number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///Type of the account that the paystub was sent to (e.g. 'checking').
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for DistributionBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}