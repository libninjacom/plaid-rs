use serde::{Serialize, Deserialize};
use super::{NumbersAch, NumbersBacs, NumbersEft, NumbersInternational};
///An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorNumber {
    ///Identifying information for transferring money to or from a US account via ACH or wire transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach: Option<NumbersAch>,
    ///Identifying information for transferring money to or from a UK bank account via BACS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<NumbersBacs>,
    ///Identifying information for transferring money to or from a Canadian bank account via EFT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eft: Option<NumbersEft>,
    ///Identifying information for transferring money to or from an international bank account via wire transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub international: Option<NumbersInternational>,
}
impl std::fmt::Display for ProcessorNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}