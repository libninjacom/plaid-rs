
use serde::{Serialize, Deserialize};
use super::{
    CreditDocumentMetadata, CreditPayStubDeductions, CreditPayStubEarnings,
    CreditPayStubEmployee, CreditPayStubEmployer, CreditPayStubNetPay,
    PayStubPayPeriodDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStub {
    pub deductions: CreditPayStubDeductions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    pub document_metadata: CreditDocumentMetadata,
    pub earnings: CreditPayStubEarnings,
    pub employee: CreditPayStubEmployee,
    pub employer: CreditPayStubEmployer,
    pub net_pay: CreditPayStubNetPay,
    pub pay_period_details: PayStubPayPeriodDetails,
}
impl std::fmt::Display for CreditPayStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}