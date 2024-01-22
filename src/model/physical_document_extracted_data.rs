use serde::{Serialize, Deserialize};
use super::IdentityVerificationDocumentAddressResponse;
///Data extracted from a user-submitted document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentExtractedData {
    /**The address extracted from the document. The address must at least contain the following fields to be a valid address: `street`, `city`, `country`. If any are missing or unable to be extracted, the address will be null.

`region`, and `postal_code` may be null based on the addressing system. For example:

Addresses from the United Kingdom will not include a region

Addresses from Hong Kong will not include postal code

Note: Optical Character Recognition (OCR) technology may sometimes extract incorrect data from a document.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<IdentityVerificationDocumentAddressResponse>,
    /**The type of identity document detected in the images provided. Will always be one of the following values:

  `drivers_license` - A driver's license issued by the associated country, establishing identity without any guarantee as to citizenship, and granting driving privileges

  `id_card` - A general national identification card, distinct from driver's licenses as it only establishes identity

  `passport` - A travel passport issued by the associated country for one of its citizens

  `residence_permit_card` - An identity document issued by the associated country permitting a foreign citizen to <em>temporarily</em> reside there

  `resident_card` - An identity document issued by the associated country permitting a foreign citizen to <em>permanently</em> reside there

  `visa` - An identity document issued by the associated country permitting a foreign citizen entry for a short duration and for a specific purpose, typically no longer than 6 months

Note: This value may be different from the ID type that the user selects within Link. For example, if they select "Driver's License" but then submit a picture of a passport, this field will say `passport`*/
    pub category: String,
    ///A date extracted from the document in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<chrono::NaiveDate>,
    ///Alpha-numeric ID number extracted via OCR from the user's document image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub issuing_country: String,
    ///An ISO 3166-2 subdivision code. Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_region: Option<String>,
}
impl std::fmt::Display for PhysicalDocumentExtractedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}