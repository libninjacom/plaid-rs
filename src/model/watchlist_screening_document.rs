use serde::{Serialize, Deserialize};
///An official document, usually issued by a governing body or institution, with an associated identifier.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningDocument {
    ///The numeric or alphanumeric identifier associated with this document.
    pub number: String,
    /**The kind of official document represented by this object.

`birth_certificate` - A certificate of birth

`drivers_license` - A license to operate a motor vehicle

`immigration_number` - Immigration or residence documents

`military_id` - Identification issued by a military group

`other` - Any document not covered by other categories

`passport` - An official passport issue by a government

`personal_identification` - Any generic personal identification that is not covered by other categories

`ration_card` - Identification that entitles the holder to rations

`ssn` - United States Social Security Number

`student_id` - Identification issued by an educational institution

`tax_id` - Identification issued for the purpose of collecting taxes

`travel_document` - Visas, entry permits, refugee documents, etc.

`voter_id` - Identification issued for the purpose of voting*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for WatchlistScreeningDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}