use serde::{Serialize, Deserialize};
///Information about the APR on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Apr {
    ///Annual Percentage Rate applied.
    pub apr_percentage: f64,
    ///The type of balance to which the APR applies.
    pub apr_type: String,
    ///Amount of money that is subjected to the APR if a balance was carried beyond payment due date. How it is calculated can vary by card issuer. It is often calculated as an average daily balance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_subject_to_apr: Option<f64>,
    ///Amount of money charged due to interest from last statement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interest_charge_amount: Option<f64>,
}
impl std::fmt::Display for Apr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}