use serde::{Serialize, Deserialize};
///An official document, usually issued by a governing body or institution, with an associated identifier.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityDocument {
    ///The numeric or alphanumeric identifier associated with this document.
    pub number: String,
    /**The kind of official document represented by this object.

`bik` - Russian bank code

`business_number` - A number that uniquely identifies the business within a category of businesses

`imo` - Number assigned to the entity by the International Maritime Organization

`other` - Any document not covered by other categories

`swift` - Number identifying a bank and branch.

`tax_id` - Identification issued for the purpose of collecting taxes*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for EntityDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}