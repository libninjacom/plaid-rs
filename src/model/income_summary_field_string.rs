use serde::{Serialize, Deserialize};
///Data about the income summary
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeSummaryFieldString {
    ///The value of the field.
    pub value: String,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: String,
}
impl std::fmt::Display for IncomeSummaryFieldString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}