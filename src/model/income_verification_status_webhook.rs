use serde::{Serialize, Deserialize};
///Fired when the status of an income verification instance has changed. It will typically take several minutes for this webhook to fire after the end user has uploaded their documents in the Document Income flow.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationStatusWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The Item ID associated with the verification.
    pub item_id: String,
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`:  The income verification processing has completed. This indicates that the documents have been parsed successfully or that the documents were not parsable. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/credit/payroll_income/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: An unexpected internal error occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: (deprecated) The income verification has been sent to the user for review.*/
    pub verification_status: String,
    ///`INCOME_VERIFICATION`
    pub webhook_code: String,
    ///`"INCOME"`
    pub webhook_type: String,
}
impl std::fmt::Display for IncomeVerificationStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}