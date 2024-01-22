use serde::{Serialize, Deserialize};
use super::LinkTokenCreateRequestEmploymentBankIncome;
///Specifies options for initializing Link for use with the Employment product. This field is required if `employment` is included in the `products` array.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestEmployment {
    ///Specifies options for initializing Link for use with Bank Employment. This field is required if `employment` is included in the `products` array and `bank` is specified in `employment_source_types`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_employment: Option<LinkTokenCreateRequestEmploymentBankIncome>,
    ///The types of source employment data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_source_types: Option<Vec<String>>,
}
impl std::fmt::Display for LinkTokenCreateRequestEmployment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}