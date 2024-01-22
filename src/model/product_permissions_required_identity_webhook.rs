use serde::{Serialize, Deserialize};
///Fired when an `ACCESS_NOT_GRANTED` error is hit for Identity. The error can be resolved by putting the user through update mode with `identity` in the `products` array, as well as through the limited beta for update mode Identity product validations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductPermissionsRequiredIdentityWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`PRODUCT_PERMISSIONS_REQUIRED`
    pub webhook_code: String,
    ///`IDENTITY`
    pub webhook_type: String,
}
impl std::fmt::Display for ProductPermissionsRequiredIdentityWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}