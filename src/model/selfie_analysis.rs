use serde::{Serialize, Deserialize};
///High level descriptions of how the associated selfie was processed. If a selfie fails verification, the details in the `analysis` object should help clarify why the selfie was rejected.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfieAnalysis {
    ///Information about the comparison between the selfie and the document (if documentary verification also ran).
    pub document_comparison: String,
}
impl std::fmt::Display for SelfieAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}