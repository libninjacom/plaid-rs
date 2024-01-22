use serde::{Serialize, Deserialize};
///Information describing a transaction category
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    ///An identifying number for the category. `category_id` is a Plaid-specific identifier and does not necessarily correspond to merchant category codes.
    pub category_id: String,
    ///`place` for physical transactions or `special` for other transactions such as bank charges.
    pub group: String,
    ///A hierarchical array of the categories to which this `category_id` belongs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy: Vec<String>,
}
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}