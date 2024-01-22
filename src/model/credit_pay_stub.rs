use serde::{Serialize, Deserialize};
use super::{
    CreditDocumentMetadata, CreditPayStubDeductions, CreditPayStubEarnings,
    CreditPayStubEmployee, CreditPayStubEmployer, CreditPayStubNetPay,
    PayStubPayPeriodDetails,
};
///An object representing an end user's pay stub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayStub {
    ///An object with the deduction information found on a pay stub.
    pub deductions: CreditPayStubDeductions,
    ///An identifier of the document referenced by the document metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    ///An object representing both a breakdown of earnings on a pay stub and the total earnings.
    pub earnings: CreditPayStubEarnings,
    ///Data about the employee.
    pub employee: CreditPayStubEmployee,
    ///Information about the employer on the pay stub.
    pub employer: CreditPayStubEmployer,
    ///An object representing information about the net pay amount on the pay stub.
    pub net_pay: CreditPayStubNetPay,
    ///Details about the pay period.
    pub pay_period_details: PayStubPayPeriodDetails,
}
impl std::fmt::Display for CreditPayStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}