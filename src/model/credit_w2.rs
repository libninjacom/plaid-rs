
use serde::{Serialize, Deserialize};
use super::{
    CreditDocumentMetadata, CreditPayStubEmployee, CreditPayStubEmployer, W2Box12,
    W2StateAndLocalWages,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditW2 {
    pub allocated_tips: Option<String>,
    #[serde(rename = "box_12")]
    pub box12: Vec<W2Box12>,
    #[serde(rename = "box_9")]
    pub box9: Option<String>,
    pub dependent_care_benefits: Option<String>,
    pub document_id: String,
    pub document_metadata: CreditDocumentMetadata,
    pub employee: CreditPayStubEmployee,
    pub employer: CreditPayStubEmployer,
    pub employer_id_number: Option<String>,
    pub federal_income_tax_withheld: Option<String>,
    pub medicare_tax_withheld: Option<String>,
    pub medicare_wages_and_tips: Option<String>,
    pub nonqualified_plans: Option<String>,
    pub other: Option<String>,
    pub retirement_plan: Option<String>,
    pub social_security_tax_withheld: Option<String>,
    pub social_security_tips: Option<String>,
    pub social_security_wages: Option<String>,
    pub state_and_local_wages: Vec<W2StateAndLocalWages>,
    pub statutory_employee: Option<String>,
    pub tax_year: Option<String>,
    pub third_party_sick_pay: Option<String>,
    pub wages_tips_other_comp: Option<String>,
}
impl std::fmt::Display for CreditW2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}