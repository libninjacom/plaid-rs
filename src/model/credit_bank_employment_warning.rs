use serde::{Serialize, Deserialize};
use super::CreditBankIncomeCause;
///The warning associated with the data that was unavailable for the Bank Employment Report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditBankEmploymentWarning {
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: CreditBankIncomeCause,
    /**The warning code identifies a specific kind of warning.
`IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item
`TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item
`ITEM_UNAPPROVED`: User exited flow before giving permission to share data for the Item
`REPORT_DELETED`: Report deleted due to customer or consumer request
`DATA_UNAVAILABLE`: No relevant data was found for the Item*/
    pub warning_code: String,
    ///The warning type which will always be `BANK_EMPLOYMENT_WARNING`.
    pub warning_type: String,
}
impl std::fmt::Display for CreditBankEmploymentWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}