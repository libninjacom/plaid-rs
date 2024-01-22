use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Statements product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestStatements {
    ///The end date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) “YYYY-MM-DD” format, e.g. "2020-10-30". If no value is provided, this will default to the current date. You can request up to two years of data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The start date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) “YYYY-MM-DD” format, e.g. "2020-10-30". If no value is provided, this will default to 3 months prior to the current date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for LinkTokenCreateRequestStatements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}