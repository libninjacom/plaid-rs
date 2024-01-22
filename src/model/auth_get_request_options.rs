use serde::{Serialize, Deserialize};
///An optional object to filter `/auth/get` results.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AuthGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}