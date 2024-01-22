use serde::{Serialize, Deserialize};
///Metadata about the application
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Application {
    ///This field will map to the application ID that is returned from /item/application/list, or provided to the institution in an oauth redirect.
    pub application_id: String,
    ///The URL for the application's website
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_url: Option<String>,
    ///A string representing the city of the client’s headquarters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///A string representing the name of client’s legal entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_legal_name: Option<String>,
    ///A string representing the country code of the client’s headquarters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    ///A human-readable name of the application for display purposes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub join_date: chrono::NaiveDate,
    ///A URL that links to the application logo image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    ///The name of the application
    pub name: String,
    ///A string representing the postal code of the client’s headquarters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///A string provided by the connected app stating why they use their respective enabled products.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_access: Option<String>,
    ///A string representing the region of the client’s headquarters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///A string representing client’s broad use case as assessed by Plaid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
}
impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}