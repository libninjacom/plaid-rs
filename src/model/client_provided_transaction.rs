use serde::{Serialize, Deserialize};
use super::ClientProvidedTransactionLocation;
///A client-provided transaction for Plaid to enrich.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedTransaction {
    ///The account subtype associated with the transaction. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_subtype: Option<String>,
    ///The account type associated with the transaction. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///The absolute value of the transaction (>= 0). When testing Enrich, note that `amount` data should be realistic. Unrealistic or inaccurate `amount` data may result in reduced quality output.
    pub amount: f64,
    ///A unique account id used to group transactions for a given account, as a unique identifier from your application. Personally identifiable information, such as an email address or phone number, should not be used in the client_account_id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_account_id: Option<String>,
    ///A unique user id used to group transactions for a given user, as a unique identifier from your application. Personally identifiable information, such as an email address or phone number, should not be used in the client_user_id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    ///The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_posted: Option<chrono::NaiveDate>,
    ///The raw description of the transaction. If you have location data in available an unstructured format, it may be appended to the `description` field.
    pub description: String,
    /**The direction of the transaction from the perspective of the account holder:

`OUTFLOW` - Includes outgoing transfers, purchases, and fees. (Typically represented as a negative value on checking accounts and debit cards and a positive value on credit cards.)

`INFLOW` - Includes incoming transfers, refunds, and income. (Typically represented as a positive value on checking accounts and debit cards and a negative value on credit cards.)*/
    pub direction: String,
    ///A unique ID for the transaction used to help you tie data back to your systems.
    pub id: String,
    ///The ISO-4217 currency code of the transaction e.g. USD.
    pub iso_currency_code: String,
    /**A representation of where a transaction took place.

Use this field to pass in structured location information you may have about your transactions. Providing location data is optional but can increase result quality. If you have unstructured location information, it may be appended to the `description` field.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ClientProvidedTransactionLocation>,
    ///Merchant category codes (MCCs) are four-digit numbers that describe a merchant's primary business activities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
}
impl std::fmt::Display for ClientProvidedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}