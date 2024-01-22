use serde::{Serialize, Deserialize};
///Result summary object specifying values for `email` attributes of risk check.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckEmail {
    ///Count of all known breaches of this email address if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breach_count: Option<i64>,
    ///Indicates whether the email address domain is custom if known, i.e. a company domain and not free or disposable.
    pub domain_is_custom: String,
    ///Indicates whether the email domain is listed as disposable if known. Disposable domains are often used to create email addresses that are part of a fake set of user details.
    pub domain_is_disposable: String,
    ///Indicates whether the email address domain is a free provider such as Gmail or Hotmail if known.
    pub domain_is_free_provider: String,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_registered_at: Option<chrono::NaiveDate>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_breached_at: Option<chrono::NaiveDate>,
    ///SMTP-MX check to confirm the email address exists if known.
    pub is_deliverable: String,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_breached_at: Option<chrono::NaiveDate>,
    ///A list of online services where this email address has been detected to have accounts or other activity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_services: Vec<String>,
    ///Indicates whether the email address top level domain, which is the last part of the domain, is fraudulent or risky if known. In most cases, a suspicious top level domain is also associated with a disposable or high-risk domain.
    pub top_level_domain_is_suspicious: String,
}
impl std::fmt::Display for RiskCheckEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}