use serde::{Serialize, Deserialize};
use super::TransferAuthorizationDecisionRationaleCode;
///Fired when Plaid is unable to originate a new ACH transaction of the recurring transfer on the planned date.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringTransferSkippedWebhook {
    /**A decision regarding the proposed transfer.

`approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `ITEM_LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub authorization_decision: String,
    /**A code representing the rationale for approving or declining the proposed transfer.

If the `rationale_code` is `null`, the transfer passed the authorization check.

Any non-`null` value for an `approved` transfer indicates that the the authorization check could not be run and that you should perform your own risk assessment on the transfer. The code will indicate why the check could not be run. Possible values for an `approved` transfer are:

`MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available.

`ITEM_LOGIN_REQUIRED` – Unable to collect the account information due to Item staleness. Can be resolved by using Link in [update mode](https://www.plaid.com/docs/link/update-mode).

`MIGRATED_ACCOUNT_ITEM` - Item created via `/transfer/account_migration` endpoint, limited information available.

`ERROR` – Unable to collect the account information due to an unspecified error.

The following codes indicate that the authorization decision was `declined`:

`NSF` – Transaction likely to result in a return due to insufficient funds.

`RISK` - Transaction is high-risk.

`TRANSFER_LIMIT_REACHED` - One or several transfer limits are reached, e.g. monthly transfer limit.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_decision_rationale_code: Option<
        TransferAuthorizationDecisionRationaleCode,
    >,
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///Plaid’s unique identifier for a recurring transfer.
    pub recurring_transfer_id: String,
    ///The planned date on which Plaid is unable to originate a new ACH transaction of the recurring transfer. This will be of the form YYYY-MM-DD.
    pub skipped_origination_date: chrono::NaiveDate,
    ///`RECURRING_TRANSFER_SKIPPED`
    pub webhook_code: String,
    ///`TRANSFER`
    pub webhook_type: String,
}
impl std::fmt::Display for RecurringTransferSkippedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}