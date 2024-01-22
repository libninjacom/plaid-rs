use serde::{Serialize, Deserialize};
///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaidError {
    /**In the Assets product, a request can pertain to more than one Item. If an error is returned for such a request, `causes` will return an array of errors containing a breakdown of these errors on the individual Item level, if any can be identified.

`causes` will only be provided for the `error_type` `ASSET_REPORT_ERROR`. `causes` will also not be populated inside an error nested within a `warning` object.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causes: Option<Vec<serde_json::Value>>,
    /**A user-friendly representation of the error code. `null` if the error is not related to user action.

This may change over time and is not safe for programmatic use.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_message: Option<String>,
    ///The URL of a Plaid documentation page with more information about the error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    ///The particular error code. Safe for programmatic use.
    pub error_code: String,
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: String,
    ///A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    ///Suggested steps for resolving the error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
}
impl std::fmt::Display for PlaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}