use serde::{Serialize, Deserialize};
///The information used to identify this loan by various parties to the transaction or other organizations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoanIdentifier {
    ///The value of the identifier for the specified type.
    #[serde(rename = "LoanIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_identifier: Option<String>,
    ///A value from a MISMO prescribed list that specifies the type of loan identifier.
    #[serde(rename = "LoanIdentifierType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_identifier_type: Option<String>,
}
impl std::fmt::Display for LoanIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}