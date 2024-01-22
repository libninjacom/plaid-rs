use serde::{Serialize, Deserialize};
use super::{Employee, PaystubEmployer, W2Box12, W2StateAndLocalWages};
///W2 is an object that represents income data taken from a W2 tax document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct W2 {
    ///Allocated tips.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_tips: Option<String>,
    #[serde(rename = "box_12")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub box12: Option<Vec<W2Box12>>,
    ///Contents from box 9 on the W2.
    #[serde(rename = "box_9")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub box9: Option<String>,
    ///Dependent care benefits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_care_benefits: Option<String>,
    ///Data about the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<Employee>,
    ///Information about the employer on the paystub
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<PaystubEmployer>,
    ///An employee identification number or EIN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer_id_number: Option<String>,
    ///Federal income tax withheld for the tax year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub federal_income_tax_withheld: Option<String>,
    ///Medicare tax withheld for the tax year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medicare_tax_withheld: Option<String>,
    ///Wages and tips from medicare.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medicare_wages_and_tips: Option<String>,
    ///Nonqualified plans.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonqualified_plans: Option<String>,
    ///Other.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    ///Retirement plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retirement_plan: Option<String>,
    ///Social security tax withheld for the tax year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_tax_withheld: Option<String>,
    ///Tips from social security.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_tips: Option<String>,
    ///Wages from social security.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_security_wages: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_and_local_wages: Option<Vec<W2StateAndLocalWages>>,
    ///Statutory employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statutory_employee: Option<String>,
    ///The tax year of the W2 document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<String>,
    ///Third party sick pay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub third_party_sick_pay: Option<String>,
    ///Wages from tips and other compensation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wages_tips_other_comp: Option<String>,
}
impl std::fmt::Display for W2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}