
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycCheckDetails {
    pub address: KycCheckAddressSummary,
    pub date_of_birth: KycCheckDateOfBirthSummary,
    pub id_number: KycCheckIdNumberSummary,
    pub name: KycCheckNameSummary,
    pub phone_number: KycCheckPhoneSummary,
    pub status: String,
}
impl std::fmt::Display for KycCheckDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}