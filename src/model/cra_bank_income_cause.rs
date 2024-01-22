use serde::{Serialize, Deserialize};
///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeCause {
    /**A user-friendly representation of the error code. null if the error is not related to user action.
This may change over time and is not safe for programmatic use.*/
    pub display_message: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. Error fields will be `null` if no error has occurred.
    pub error_code: String,
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: String,
}
impl std::fmt::Display for CraBankIncomeCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}