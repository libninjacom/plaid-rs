use serde::{Serialize, Deserialize};
use super::PlaidError;
///The `USER_PERMISSION_REVOKED` webhook may be fired when an end user has used either the [my.plaid.com portal](https://my.plaid.com) or the financial institution’s OAuth consent portal to revoke the permission that they previously granted to access an Item. This webhook is not guaranteed to always be fired upon consent revocation, since some institutions’ consent portals do not trigger this webhook. Once access to an Item has been revoked, it cannot be restored. If the user subsequently returns to your application, a new Item must be created for the user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPermissionRevokedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`USER_PERMISSION_REVOKED`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for UserPermissionRevokedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}