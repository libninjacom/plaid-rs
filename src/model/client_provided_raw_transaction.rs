use serde::{Serialize, Deserialize};
///A client-provided transaction for Plaid to enhance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClientProvidedRawTransaction {
    /**The value of the transaction with direction. (NOTE: this will affect enrichment results, so directions are important):.
  Negative (-) for credits (e.g., incoming transfers, refunds)
  Positive (+) for debits (e.g., purchases, fees, outgoing transfers)*/
    pub amount: f64,
    ///The raw description of the transaction.
    pub description: String,
    ///A unique ID for the transaction used to help you tie data back to your systems.
    pub id: String,
    ///The ISO-4217 currency code of the transaction e.g. USD.
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedRawTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}