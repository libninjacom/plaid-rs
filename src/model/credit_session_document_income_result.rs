use serde::{Serialize, Deserialize};
///The details of a document income verification in Link
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditSessionDocumentIncomeResult {
    ///The number of bank statements uploaded by the user.
    pub num_bank_statements_uploaded: i64,
    ///The number of paystubs uploaded by the user.
    pub num_paystubs_uploaded: i64,
    ///The number of w2s uploaded by the user.
    #[serde(rename = "num_w2s_uploaded")]
    pub num_w2_s_uploaded: i64,
}
impl std::fmt::Display for CreditSessionDocumentIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}