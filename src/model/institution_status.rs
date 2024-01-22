use serde::{Serialize, Deserialize};
use super::{HealthIncident, ProductStatus};
/**The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.

Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionStatus {
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<ProductStatus>,
    ///Details of recent health incidents associated with the institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_incidents: Option<Vec<HealthIncident>>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub investments: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub investments_updates: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_logins: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liabilities: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liabilities_updates: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions_updates: Option<ProductStatus>,
}
impl std::fmt::Display for InstitutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}