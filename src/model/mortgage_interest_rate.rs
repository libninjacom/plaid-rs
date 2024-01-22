use serde::{Serialize, Deserialize};
///Object containing metadata about the interest rate for the mortgage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MortgageInterestRate {
    ///Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
    ///The type of interest charged (fixed or variable).
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for MortgageInterestRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}