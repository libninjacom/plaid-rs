use serde::{Serialize, Deserialize};
use super::{Counterparty, Location, PersonalFinanceCategory};
///A grouping of the Plaid produced transaction enhancement fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Enhancements {
    ///A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<String>,
    ///The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    ///The check number of the transaction. This field is only populated for check transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    ///The counterparties present in the transaction. Counterparties, such as the merchant or the financial institution, are extracted by Plaid from the raw description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<Counterparty>>,
    ///A representation of where a transaction took place
    pub location: Location,
    ///The URL of a logo associated with this transaction, if available. The logo will always be 100×100 pixel PNG file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    ///The name of the primary counterparty, such as the merchant or the financial institution, as extracted by Plaid from the raw description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /**The channel used to make a payment.
`online:` transactions that took place online.

`in store:` transactions that were made at a physical location.

`other:` transactions that relate to banks, e.g. fees or deposits.*/
    pub payment_channel: String,
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy CSV file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories. If you are migrating to personal finance categories from the legacy categories, also refer to the [`migration guide`](https://plaid.com/docs/transactions/pfc-migration/).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    ///The URL of an icon associated with the primary personal finance category. The icon will always be 100×100 pixel PNG file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_icon_url: Option<String>,
    ///The website associated with this transaction, if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for Enhancements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}