use serde::{Serialize, Deserialize};
///Student loan repayment information used to configure Sandbox test data for the Liabilities product
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentLoanRepaymentModel {
    ///Configures the number of months before repayment starts.
    pub non_repayment_months: f64,
    ///Configures the number of months of repayments before the loan is paid off.
    pub repayment_months: f64,
    ///The only currently supported value for this field is `standard`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for StudentLoanRepaymentModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}