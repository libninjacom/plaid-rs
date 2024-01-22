use serde::{Serialize, Deserialize};
use super::PaystubPayFrequency;
///An object representing details that can be found on the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubDetails {
    ///Pay date on the paystub in the 'YYYY-MM-DD' format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<chrono::NaiveDate>,
    ///The frequency at which the employee is paid. Possible values: `MONTHLY`, `BI-WEEKLY`, `WEEKLY`, `SEMI-MONTHLY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<PaystubPayFrequency>,
    ///Ending date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_end_date: Option<chrono::NaiveDate>,
    ///Beginning date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_start_date: Option<chrono::NaiveDate>,
    ///The name of the payroll provider that generated the paystub, e.g. ADP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paystub_provider: Option<String>,
}
impl std::fmt::Display for PaystubDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}