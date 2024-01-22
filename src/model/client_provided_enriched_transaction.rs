use serde::{Serialize, Deserialize};
use super::Enrichments;
///A client-provided transaction that Plaid has enriched.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedEnrichedTransaction {
    ///The account subtype associated with the transaction. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_subtype: Option<String>,
    ///The account type associated with the transaction. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///The absolute value of the transaction (>= 0)
    pub amount: f64,
    ///A unique account id used to group transactions for a given account, as a unique identifier from your application. Personally identifiable information, such as an email address or phone number, should not be used in the client_account_id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_account_id: Option<String>,
    ///A unique user id used to group transactions for a given user, as a unique identifier from your application. Personally identifiable information, such as an email address or phone number, should not be used in the client_user_id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    ///The raw description of the transaction.
    pub description: String,
    /**The direction of the transaction from the perspective of the account holder:

`OUTFLOW` - Includes outgoing transfers, purchases, and fees. (Typically represented as a negative value on checking accounts and debit cards and a positive value on credit cards.)

`INFLOW` - Includes incoming transfers, refunds, and income. (Typically represented as a positive value on checking accounts and debit cards and a negative value on credit cards.)*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    ///A grouping of the Plaid produced transaction enrichment fields.
    pub enrichments: Enrichments,
    ///The unique ID for the transaction as provided by you in the request.
    pub id: String,
    ///The ISO-4217 currency code of the transaction e.g. USD.
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedEnrichedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}