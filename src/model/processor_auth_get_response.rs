use serde::{Serialize, Deserialize};
use super::{AccountBase, ProcessorNumber};
///ProcessorAuthGetResponse defines the response schema for `/processor/auth/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorAuthGetResponse {
    ///A single account at a financial institution.
    pub account: AccountBase,
    ///An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.
    pub numbers: ProcessorNumber,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorAuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}