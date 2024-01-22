use serde::{Serialize, Deserialize};
use super::SandboxPublicTokenCreateRequestIncomeVerificationBankIncome;
///A set of parameters for income verification options. This field is required if `income_verification` is included in the `initial_products` array.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    ///Specifies options for Bank Income. This field is required if `income_verification` is included in the `initial_products` array and `bank` is specified in `income_source_types`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<SandboxPublicTokenCreateRequestIncomeVerificationBankIncome>,
    ///The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_source_types: Option<Vec<String>>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}