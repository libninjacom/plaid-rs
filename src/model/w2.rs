
use serde::{Serialize, Deserialize};
use super::{Employee, PaystubEmployer, W2Box12, W2StateAndLocalWages};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct W2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_tips: Option<String>,
    #[serde(rename = "box_12")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box12: Option<Vec<W2Box12>>,
    #[serde(rename = "box_9")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box9: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_care_benefits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<Employee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer: Option<PaystubEmployer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federal_income_tax_withheld: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medicare_tax_withheld: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medicare_wages_and_tips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonqualified_plans: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retirement_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_tax_withheld: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_tips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_wages: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_and_local_wages: Option<Vec<W2StateAndLocalWages>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statutory_employee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_sick_pay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wages_tips_other_comp: Option<String>,
}
impl std::fmt::Display for W2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}