use serde::{Serialize, Deserialize};
use super::{
    LinkTokenCreateRequestIncomeVerificationBankIncome,
    LinkTokenCreateRequestIncomeVerificationPayrollIncome,
    LinkTokenCreateRequestUserStatedIncomeSource,
};
///Specifies options for initializing Link for use with the Income product. This field is required if `income_verification` is included in the `products` array.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerification {
    /**An array of access tokens corresponding to Items that a user has previously connected with. Data from these institutions will be cross-referenced with document data received during the Document Income flow to help verify that the uploaded documents are accurate. If the `transactions` product was not initialized for these Items during link, it will be initialized after this Link session.

This field should only be used with the `payroll` income source type.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_tokens: Option<Vec<String>>,
    ///The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_report_id: Option<String>,
    ///Specifies options for initializing Link for use with Bank Income. This field is required if `income_verification` is included in the `products` array and `bank` is specified in `income_source_types`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income: Option<LinkTokenCreateRequestIncomeVerificationBankIncome>,
    ///The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_source_types: Option<Vec<String>>,
    ///The `income_verification_id` of the verification instance, as provided by `/income/verification/create`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_verification_id: Option<String>,
    ///Specifies options for initializing Link for use with Payroll Income (including Document Income). Further customization options for Document Income, such as customizing which document types may be uploaded, are also available via the [Link Customization pane](https://dashboard.plaid.com/link) in the Dashboard. (Requires Production enablement.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_income: Option<LinkTokenCreateRequestIncomeVerificationPayrollIncome>,
    ///A list of user stated income sources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stated_income_sources: Option<Vec<LinkTokenCreateRequestUserStatedIncomeSource>>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}