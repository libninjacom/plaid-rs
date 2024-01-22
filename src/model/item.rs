use serde::{Serialize, Deserialize};
use super::PlaidError;
///Metadata about the Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    ///A list of products available for the Item that have not yet been accessed. The contents of this array will be mutually exclusive with `billed_products`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_products: Vec<String>,
    ///A list of products that have been billed for the Item. The contents of this array will be mutually exclusive with `available_products`. Note - `billed_products` is populated in all environments but only requests in Production are billed. Also note that products that are billed on a pay-per-call basis rather than a pay-per-Item basis, such as `balance`, will not appear here.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub billed_products: Vec<String>,
    /**The RFC 3339 timestamp after which the consent provided by the end user will expire. Upon consent expiration, the item will enter the `ITEM_LOGIN_REQUIRED` error state. To circumvent the `ITEM_LOGIN_REQUIRED` error and maintain continuous consent, the end user can reauthenticate via Link’s update mode in advance of the consent expiration time.

Note - This is only relevant for certain OAuth-based institutions. For all other institutions, this field will be null.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    ///A list of products that have gone through consent collection for the Item. Only present for those enabled in the [Data Transparency](https://plaid.com/docs/link/data-transparency-messaging-migration-guide) beta. If you are not enrolled in Data Transparency, this field is not used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consented_products: Option<Vec<String>>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The Plaid Institution ID associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    ///A list of products added to the Item. In almost all cases, this will be the same as the `billed_products` field. For some products, it is possible for the product to be added to an Item but not yet billed (e.g. Assets, before `/asset_report/create` has been called, or Auth or Identity when added as Optional Products but before their endpoints have been called), in which case the product may appear in `products` but not in `billed_products`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    /**Indicates whether an Item requires user interaction to be updated, which can be the case for Items with some forms of two-factor authentication.

`background` - Item can be updated in the background

`user_present_required` - Item requires user interaction to be updated*/
    pub update_type: String,
    ///The URL registered to receive webhooks for the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}