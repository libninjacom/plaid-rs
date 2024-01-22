use serde::{Serialize, Deserialize};
///An optional object containing payment details. If set, a payment risk assessment is performed and returned as AccountsBalanceGetResponsePaymentRiskAssessment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsBalanceGetRequestPaymentDetails {
    /**The Plaid `account_id` of the account that is the funding source for the proposed transaction. The `account_id` is returned in the `/accounts/get` endpoint as well as the [`onSuccess`](/docs/link/ios/#link-ios-onsuccess-linkSuccess-metadata-accounts-id) callback metadata.

This will return an [`INVALID_ACCOUNT_ID`](/docs/errors/invalid-input/#invalid_account_id) error if the account has been removed at the bank or if the `account_id` is no longer valid.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The transaction amount, in USD (e.g. `102.05`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    ///The threshold percentage of the account balance used for comparison with the requested ACH debit amount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_threshold_percentage: Option<i64>,
    ///The unique ID that you would like to use to refer to this transaction. For your convenience mapping your internal data, you could use your internal identifier for this transaction. The max length for this field is 36 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_transaction_id: Option<String>,
}
impl std::fmt::Display for AccountsBalanceGetRequestPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}