use serde::{Serialize, Deserialize};
use super::{
    KycCheckAddressSummary, KycCheckDateOfBirthSummary, KycCheckIdNumberSummary,
    KycCheckNameSummary, KycCheckPhoneSummary,
};
///Additional information for the `kyc_check` step. This field will be `null` unless `steps.kyc_check` has reached a terminal state of either `success` or `failed`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KycCheckDetails {
    ///Result summary object specifying how the `address` field matched.
    pub address: KycCheckAddressSummary,
    ///Result summary object specifying how the `date_of_birth` field matched.
    pub date_of_birth: KycCheckDateOfBirthSummary,
    ///Result summary object specifying how the `id_number` field matched.
    pub id_number: KycCheckIdNumberSummary,
    ///Result summary object specifying how the `name` field matched.
    pub name: KycCheckNameSummary,
    ///Result summary object specifying how the `phone` field matched.
    pub phone_number: KycCheckPhoneSummary,
    ///The outcome status for the associated Identity Verification attempt's `kyc_check` step. This field will always have the same value as `steps.kyc_check`.
    pub status: String,
}
impl std::fmt::Display for KycCheckDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}