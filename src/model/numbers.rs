use serde::{Serialize, Deserialize};
///Account and bank identifier number data used to configure the test account. All values are optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Numbers {
    ///Will be used for the account number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    ///Must be a valid ACH routing number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    ///Must be a valid wire transfer routing number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_wire_routing: Option<String>,
    ///BACS sort code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs_sort_code: Option<String>,
    ///EFT branch number. Must be specified alongside `eft_institution`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eft_branch: Option<String>,
    ///EFT institution number. Must be specified alongside `eft_branch`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eft_institution: Option<String>,
    ///Bank identifier code (BIC). Must be specified alongside `international_iban`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_bic: Option<String>,
    ///International bank account number (IBAN). If no account number is specified via `account`, will also be used as the account number by default. Must be specified alongside `international_bic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international_iban: Option<String>,
}
impl std::fmt::Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}