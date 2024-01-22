use serde::{Serialize, Deserialize};
use super::{AuthMetadata, InstitutionStatus, PaymentInitiationMetadata};
///Details relating to a specific financial institution
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Institution {
    ///Metadata that captures information about the Auth features of an institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_metadata: Option<AuthMetadata>,
    ///A list of the country codes supported by the institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_codes: Vec<String>,
    ///A partial list of DTC numbers associated with the institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dtc_numbers: Option<Vec<String>>,
    ///Unique identifier for the institution. Note that the same institution may have multiple records, each with different institution IDs; for example, if the institution has migrated to OAuth, there may be separate `institution_id`s for the OAuth and non-OAuth versions of the institution. Institutions that operate in different countries or with multiple login portals may also have separate `institution_id`s for each country or portal.
    pub institution_id: String,
    ///Base64 encoded representation of the institution's logo, returned as a base64 encoded 152x152 PNG. Not all institutions' logos are available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    ///The official name of the institution.
    pub name: String,
    ///Indicates that the institution has an OAuth login flow. This will be `true` if OAuth is supported for any Items associated with the institution, even if the institution also supports non-OAuth connections.
    pub oauth: bool,
    ///Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_initiation_metadata: Option<PaymentInitiationMetadata>,
    ///Hexadecimal representation of the primary color used by the institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    ///A list of the Plaid products supported by the institution. Note that only institutions that support Instant Auth will return `auth` in the product array; institutions that do not list `auth` may still support other Auth methods such as Instant Match or Automated Micro-deposit Verification. To identify institutions that support those methods, use the `auth_metadata` object. For more details, see [Full Auth coverage](https://plaid.com/docs/auth/coverage/).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    ///A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routing_numbers: Vec<String>,
    /**The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.

Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstitutionStatus>,
    ///The URL for the institution's website
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for Institution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}