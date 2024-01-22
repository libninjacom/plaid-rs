use serde::{Serialize, Deserialize};
///A statement's metadata associated with an account
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementsStatement {
    ///Month of the year. Possible values: 1 through 12 (January through December).
    pub month: i64,
    ///Plaid's unique identifier for the statement.
    pub statement_id: String,
    ///The year of statement.
    pub year: i64,
}
impl std::fmt::Display for StatementsStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}