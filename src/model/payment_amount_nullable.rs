use serde::{Serialize, Deserialize};
///The amount and currency of a payment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentAmountNullable {
    ///The ISO-4217 currency code of the payment. For standing orders and payment consents, `"GBP"` must be used. For Poland, Denmark, Sweden and Norway, only the local currency is currently supported.
    pub currency: String,
    ///The amount of the payment. Must contain at most two digits of precision e.g. `1.23`.
    pub value: f64,
}
impl std::fmt::Display for PaymentAmountNullable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}