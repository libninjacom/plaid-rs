use serde::{Serialize, Deserialize};
use super::{CraBankIncomeTransaction, CreditAmountWithCurrency};
///The end user's monthly summary for the income source(s).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeHistoricalSummary {
    /**The end date of the period included in this monthly summary.
This date will be the last day of the month, unless the month being covered is a partial month because it is the last month included in the summary and the date range being requested does not end with the last day of the month.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    /**The start date of the period covered in this monthly summary.
This date will be the first day of the month, unless the month being covered is a partial month because it is the first month included in the summary and the date range being requested does not begin with the first day of the month.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    /**Total amount of earnings for the income source(s) of the user for the month in the summary.
This can contain multiple amounts, with each amount denominated in one unique currency.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amounts: Option<Vec<CreditAmountWithCurrency>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<CraBankIncomeTransaction>>,
}
impl std::fmt::Display for CraBankIncomeHistoricalSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}