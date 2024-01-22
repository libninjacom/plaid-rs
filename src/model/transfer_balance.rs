use serde::{Serialize, Deserialize};
///Information about the balance held with Plaid.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferBalance {
    ///The amount of this balance available for use (decimal string with two digits of precision e.g. "10.00").
    pub available: String,
    ///The available balance, plus amount of pending funds that in processing (decimal string with two digits of precision e.g. "10.00").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    /**The type of balance.

`prefunded_rtp_credits` - Your prefunded RTP credit balance with Plaid
`prefunded_ach_credits` - Your prefunded ACH credit balance with Plaid*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TransferBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}