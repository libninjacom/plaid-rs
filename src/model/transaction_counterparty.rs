use serde::{Serialize, Deserialize};
///The counterparty, such as the merchant or financial institution, is extracted by Plaid from the raw description.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionCounterparty {
    /**A description of how confident we are that the provided counterparty is involved in the transaction.

`VERY_HIGH`: We recognize this counterparty and we are more than 98% confident that it is involved in this transaction.
`HIGH`: We recognize this counterparty and we are more than 90% confident that it is involved in this transaction.
`MEDIUM`: We are moderately confident that this counterparty was involved in this transaction, but some details may differ from our records.
`LOW`: We didn’t find a matching counterparty in our records, so we are returning a cleansed name parsed out of the request description.
`UNKNOWN`: We don’t know the confidence level for this counterparty.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
    ///A unique, stable, Plaid-generated ID that maps to the counterparty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    ///The URL of a logo associated with the counterparty, if available. The logo will always be 100×100 pixel PNG file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    ///The name of the counterparty, such as the merchant or the financial institution, as extracted by Plaid from the raw description.
    pub name: String,
    /**The counterparty type.

`merchant`: a provider of goods or services for purchase
`financial_institution`: a financial entity (bank, credit union, BNPL, fintech)
`payment_app`: a transfer or P2P app (e.g. Zelle)
`marketplace`: a marketplace (e.g DoorDash, Google Play Store)
`payment_terminal`: a point-of-sale payment terminal (e.g Square, Toast)
`income_source`: the payer in an income transaction (e.g., an employer, client, or government agency)*/
    #[serde(rename = "type")]
    pub type_: String,
    ///The website associated with the counterparty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for TransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}