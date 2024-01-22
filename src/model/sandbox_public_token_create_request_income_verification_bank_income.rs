use serde::{Serialize, Deserialize};
///Specifies options for Bank Income. This field is required if `income_verification` is included in the `initial_products` array and `bank` is specified in `income_source_types`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestIncomeVerificationBankIncome {
    ///The number of days of data to request for the Bank Income product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestIncomeVerificationBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}