use serde::{Serialize, Deserialize};
///DepositSwitchGetResponse defines the response schema for `/deposit_switch/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchGetResponse {
    ///When `true`, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always `null` if the deposit switch has not been completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_has_multiple_allocations: Option<bool>,
    ///The dollar amount of direct deposit allocated to the target account. Always `null` if the target account is not allocated an amount or if the deposit switch has not been completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_allocated: Option<f64>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was completed. Always `null` if the deposit switch has not been completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<chrono::NaiveDate>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was created.
    pub date_created: chrono::NaiveDate,
    ///The ID of the deposit switch.
    pub deposit_switch_id: String,
    ///The ID of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer_id: Option<String>,
    ///The name of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer_name: Option<String>,
    ///The ID of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The name of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    ///When `true`, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When `false`, user’s direct deposit is allocated as a percent or amount. Always `null` if the deposit switch has not been completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_allocated_remainder: Option<bool>,
    ///The percentage of direct deposit allocated to the target account. Always `null` if the target account is not allocated a percentage or if the deposit switch has not been completed or if `is_allocated_remainder` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_allocated: Option<f64>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**The state, or status, of the deposit switch.

- `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

- `processing` – The deposit switch request has been submitted and is being processed.

- `completed` – The user's employer has fulfilled the deposit switch request.

- `error` – There was an error processing the deposit switch request.*/
    pub state: String,
    /**The method used to make the deposit switch.

- `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.

- `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.

- `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.'*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_method: Option<serde_json::Value>,
    ///The ID of the bank account the direct deposit was switched to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
    ///The ID of the Item the direct deposit was switched to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_item_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}