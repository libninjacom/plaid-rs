use serde::{Serialize, Deserialize};
use super::{RiskProfile, SignalEvaluateCoreAttributes, SignalScores, SignalWarning};
///SignalEvaluateResponse defines the response schema for `/signal/income/evaluate`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalEvaluateResponse {
    /**The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:

`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
`is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account

For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core_attributes: Option<SignalEvaluateCoreAttributes>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///Details about the transaction result after evaluated by the requested risk profile. If a `risk_profile_key` is not provided, this field will be omitted. This feature is currently in closed beta; to request access, contact your account manager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_profile: Option<RiskProfile>,
    ///Risk scoring details broken down by risk category.
    pub scores: SignalScores,
    ///If bank information was not available to be used in the Signal model, this array contains warnings describing why bank data is missing. If you want to receive an API error instead of Signal scores in the case of missing bank data, file a support ticket or contact your Plaid account manager.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<SignalWarning>,
}
impl std::fmt::Display for SignalEvaluateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}