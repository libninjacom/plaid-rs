use serde::{Serialize, Deserialize};
///Score found by matching address provided by the API with the address on the account at the financial institution. The score can range from 0 to 100 where 100 is a perfect match and 0 is a no match. If the account contains multiple owners, the maximum match score is filled.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressMatchScore {
    ///postal code was provided for both and was a match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_postal_code_match: Option<bool>,
    ///Match score for address. 100 is a perfect match, 99-90 is a strong match, 89-70 is a partial match, anything below 70 is considered a weak match. Typically, the match threshold should be set to a score of 70 or higher. If the address is missing from either the API or financial institution, this is null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}
impl std::fmt::Display for AddressMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}