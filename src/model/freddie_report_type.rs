
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum FreddieReportType {
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT")]
    VerificationOfEmployment,
}