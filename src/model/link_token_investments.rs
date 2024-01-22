use serde::{Serialize, Deserialize};
///Configuration parameters for the Investments product
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenInvestments {
    ///If `true`, allow users to manually enter Investments account and holdings information. Defaults to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_manual_entry: Option<bool>,
    ///If `true`, allow self-custody crypto wallets to be added without requiring signature verification. Defaults to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_unverified_crypto_wallets: Option<bool>,
}
impl std::fmt::Display for LinkTokenInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}