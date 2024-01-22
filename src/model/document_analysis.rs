use serde::{Serialize, Deserialize};
use super::PhysicalDocumentExtractedDataAnalysis;
///High level descriptions of how the associated document was processed. If a document fails verification, the details in the `analysis` object should help clarify why the document was rejected.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentAnalysis {
    /**High level summary of whether the document in the provided image matches the formatting rules and security checks for the associated jurisdiction.

For example, most identity documents have formatting rules like the following:


The image of the person's face must have a certain contrast in order to highlight skin tone


The subject in the document's image must remove eye glasses and pose in a certain way


The informational fields (name, date of birth, ID number, etc.) must be colored and aligned according to specific rules


Security features like watermarks and background patterns must be present

So a `match` status for this field indicates that the document in the provided image seems to conform to the various formatting and security rules associated with the detected document.*/
    pub authenticity: String,
    ///Analysis of the data extracted from the submitted document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extracted_data: Option<PhysicalDocumentExtractedDataAnalysis>,
    /**A high level description of the quality of the image the user submitted.

For example, an image that is blurry, distorted by glare from a nearby light source, or improperly framed might be marked as low or medium quality. Poor quality images are more likely to fail OCR and/or template conformity checks.

Note: By default, Plaid will let a user recapture document images twice before failing the entire session if we attribute the failure to low image quality.*/
    pub image_quality: String,
}
impl std::fmt::Display for DocumentAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}