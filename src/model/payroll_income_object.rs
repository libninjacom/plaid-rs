use serde::{Serialize, Deserialize};
use super::{Credit1099, CreditPayStub, CreditW2};
///An object representing payroll data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollIncomeObject {
    ///ID of the payroll provider account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///Array of tax form 1099s.
    #[serde(rename = "form1099s")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub form1099_s: Vec<Credit1099>,
    ///Array of pay stubs for the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pay_stubs: Vec<CreditPayStub>,
    ///Array of tax form W-2s.
    #[serde(rename = "w2s")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub w2_s: Vec<CreditW2>,
}
impl std::fmt::Display for PayrollIncomeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}