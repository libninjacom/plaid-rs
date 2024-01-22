use serde::{Serialize, Deserialize};
///Fired when an Item has exited the `ITEM_LOGIN_REQUIRED` state without the user having gone through the update mode flow in your app (this can happen if the user completed the update mode in a different app). If you have messaging that tells the user to complete the update mode flow, you should silence this messaging upon receiving the `LOGIN_REPAIRED` webhook.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemLoginRepairedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`LOGIN_REPAIRED`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for ItemLoginRepairedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}