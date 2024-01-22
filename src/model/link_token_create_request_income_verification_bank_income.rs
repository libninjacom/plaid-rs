use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with Bank Income. This field is required if `income_verification` is included in the `products` array and `bank` is specified in `income_source_types`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerificationBankIncome {
    ///The number of days of data to request for the Bank Income product
    pub days_requested: i64,
    ///Whether to enable multiple Items to be added in the Link session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_multiple_items: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}