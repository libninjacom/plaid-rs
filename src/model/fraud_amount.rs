use serde::{Serialize, Deserialize};
/**The amount and currency of the fraud or attempted fraud.
`fraud_amount` should be omitted to indicate an unknown fraud amount.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FraudAmount {
    ///An ISO-4217 currency code.
    pub iso_currency_code: String,
    /**The amount value.
This value can be 0 to indicate no money was lost.
Must not contain more than two digits of precision (e.g., `1.23`).*/
    pub value: f64,
}
impl std::fmt::Display for FraudAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}