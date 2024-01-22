use serde::{Serialize, Deserialize};
/**The address extracted from the document. The address must at least contain the following fields to be a valid address: `street`, `city`, `country`. If any are missing or unable to be extracted, the address will be null.

`region`, and `postal_code` may be null based on the addressing system. For example:

Addresses from the United Kingdom will not include a region

Addresses from Hong Kong will not include postal code

Note: Optical Character Recognition (OCR) technology may sometimes extract incorrect data from a document.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationDocumentAddressResponse {
    ///City extracted from the document.
    pub city: String,
    ///Valid, capitalized, two-letter ISO code representing the country extracted from the document. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
    ///The postal code extracted from the document. Between 2 and 10 alphanumeric characters. For US-based addresses this must be 5 numeric digits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///An ISO 3166-2 subdivision code extracted from the document. Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The full street address extracted from the document.
    pub street: String,
}
impl std::fmt::Display for IdentityVerificationDocumentAddressResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}