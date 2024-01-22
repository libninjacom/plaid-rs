use serde::{Serialize, Deserialize};
use super::{NumbersAch, NumbersBacs, NumbersEft, NumbersInternational};
///An object containing identifying numbers used for making electronic transfers to and from the `accounts`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by any `accounts` for which data has been requested, the array for that type will be empty.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthGetNumbers {
    ///An array of ACH numbers identifying accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ach: Vec<NumbersAch>,
    ///An array of BACS numbers identifying accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bacs: Vec<NumbersBacs>,
    ///An array of EFT numbers identifying accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub eft: Vec<NumbersEft>,
    ///An array of IBAN numbers identifying accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub international: Vec<NumbersInternational>,
}
impl std::fmt::Display for AuthGetNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}