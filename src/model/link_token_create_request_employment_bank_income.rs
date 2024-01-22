use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with Bank Employment. This field is required if `employment` is included in the `products` array and `bank` is specified in `employment_source_types`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestEmploymentBankIncome {
    ///The number of days of data to request for the Bank Employment product.
    pub days_requested: i64,
}
impl std::fmt::Display for LinkTokenCreateRequestEmploymentBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}