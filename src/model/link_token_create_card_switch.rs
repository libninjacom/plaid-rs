use serde::{Serialize, Deserialize};
///A map containing data to pass in for the Card Switch flow.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateCardSwitch {
    ///The BIN (Bank Identification Number) of the card to switch.
    pub card_bin: String,
}
impl std::fmt::Display for LinkTokenCreateCardSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}