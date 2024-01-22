use serde::{Serialize, Deserialize};
///Analysis of the data extracted from the submitted document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentExtractedDataAnalysis {
    ///A match summary describing the cross comparison between the subject's date of birth, extracted from the document image, and the date of birth they separately provided to the identity verification attempt.
    pub date_of_birth: String,
    /**A description of whether the associated document was expired when the verification was performed.

Note: In the case where an expiration date is not present on the document or failed to be extracted, this value will be `no_data`.*/
    pub expiration_date: String,
    /**A binary match indicator specifying whether the country that issued the provided document matches the country that the user separately provided to Plaid.

Note: You can configure whether a `no_match` on `issuing_country` fails the `documentary_verification` by editing your Plaid Template.*/
    pub issuing_country: String,
    ///A match summary describing the cross comparison between the subject's name, extracted from the document image, and the name they separately provided to identity verification attempt.
    pub name: String,
}
impl std::fmt::Display for PhysicalDocumentExtractedDataAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}