use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with Payroll Income (including Document Income). Further customization options for Document Income, such as customizing which document types may be uploaded, are also available via the [Link Customization pane](https://dashboard.plaid.com/link) in the Dashboard. (Requires Production enablement.)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    ///The types of payroll income verification to enable for the Link session. If none are specified, then users will see both document and digital payroll income.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_types: Option<Vec<String>>,
    ///An identifier to indicate whether the income verification Link token will be used for update mode. This field is only relevant for participants in the Payroll Income Refresh beta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_update_mode: Option<bool>,
    ///Uniquely identify a payroll income Item to update with.  This field is only relevant for participants in the Payroll Income Refresh beta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id_to_update: Option<String>,
    ///The types of analysis to enable for document uploads. If this field is not provided, then docs will undergo OCR parsing only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parsing_config: Option<Vec<String>>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}