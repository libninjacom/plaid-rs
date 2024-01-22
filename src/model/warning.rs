use serde::{Serialize, Deserialize};
use super::Cause;
///It is possible for an Asset Report to be returned with missing account owner information. In such cases, the Asset Report will contain warning data in the response, indicating why obtaining the owner information failed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Warning {
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: Cause,
    ///The warning code identifies a specific kind of warning. `OWNERS_UNAVAILABLE` indicates that account-owner information is not available.`INVESTMENTS_UNAVAILABLE` indicates that Investments specific information is not available. `TRANSACTIONS_UNAVAILABLE` indicates that transactions information associated with Credit and Depository accounts are unavailable.
    pub warning_code: String,
    ///The warning type, which will always be `ASSET_REPORT_WARNING`
    pub warning_type: String,
}
impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}