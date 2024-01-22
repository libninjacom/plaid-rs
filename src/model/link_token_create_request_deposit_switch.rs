use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Deposit Switch (beta) product. This field is required if `deposit_switch` is included in the `products` array.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestDepositSwitch {
    ///The `deposit_switch_id` provided by the `/deposit_switch/create` endpoint.
    pub deposit_switch_id: String,
}
impl std::fmt::Display for LinkTokenCreateRequestDepositSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}