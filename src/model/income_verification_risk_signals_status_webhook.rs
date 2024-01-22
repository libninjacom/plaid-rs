use serde::{Serialize, Deserialize};
///Fired when risk signals have been processed for documents uploaded via Document Income. It will typically take a minute or two for this webhook to fire after the end user has uploaded their documents in the Document Income flow. Once this webhook has fired, `/credit/payroll_income/risk_signals/get` may then be called to determine whether the documents were successfully processed and to retrieve risk data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationRiskSignalsStatusWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The Item ID associated with the verification.
    pub item_id: String,
    ///`RISK_SIGNALS_PROCESSING_COMPLETE`: The income verification fraud detection processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/credit/payroll_income/risk_signals/get` endpoint to get all risk signal data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_signals_status: Option<String>,
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    ///`INCOME_VERIFICATION_RISK_SIGNALS`
    pub webhook_code: String,
    ///`"INCOME"`
    pub webhook_type: String,
}
impl std::fmt::Display for IncomeVerificationRiskSignalsStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}