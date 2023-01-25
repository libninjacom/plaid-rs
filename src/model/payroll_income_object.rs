
use serde::{Serialize, Deserialize};
use super::{Credit1099, CreditPayStub, CreditW2};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollIncomeObject {
    pub account_id: Option<String>,
    #[serde(rename = "form1099s")]
    pub form1099_s: Vec<Credit1099>,
    pub pay_stubs: Vec<CreditPayStub>,
    #[serde(rename = "w2s")]
    pub w2_s: Vec<CreditW2>,
}
impl std::fmt::Display for PayrollIncomeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}