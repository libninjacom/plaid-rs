use serde::{Serialize, Deserialize};
///Score found by matching name provided by the API with the name on the account at the financial institution. If the account contains multiple owners, the maximum match score is filled.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NameMatchScore {
    ///Is `true` if the name on either of the names that was matched for the score contained strings indicative of a business name, such as "CORP", "LLC", "INC", or "LTD". A `true` result generally indicates the entity is a business. However, a `false` result does not mean the entity is not a business, as some businesses do not use these strings in the names used for their financial institution accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_business_name_detected: Option<bool>,
    ///first or last name completely matched, likely a family member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_first_name_or_last_name_match: Option<bool>,
    ///nickname matched, example Jennifer and Jenn.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_nickname_match: Option<bool>,
    ///Match score for name. 100 is a perfect score, 99-85 means a strong match, 84-70 is a partial match, any score less than 70 is a mismatch. Typically, the match threshold should be set to a score of 70 or higher. If the name is missing from either the API or financial institution, this is null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}
impl std::fmt::Display for NameMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}