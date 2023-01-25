
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum TaxpayerIdentifierType {
    IndividualTaxpayerIdentificationNumber,
    SocialSecurityNumber,
}