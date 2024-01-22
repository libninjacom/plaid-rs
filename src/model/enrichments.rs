use serde::{Serialize, Deserialize};
use super::{Counterparty, Location, PersonalFinanceCategory, Recurrence};
///A grouping of the Plaid produced transaction enrichment fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Enrichments {
    ///The check number of the transaction. This field is only populated for check transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    ///The counterparties present in the transaction. Counterparties, such as the merchant or the financial institution, are extracted by Plaid from the raw description.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub counterparties: Vec<Counterparty>,
    ///A unique, stable, Plaid-generated ID that maps to the primary counterparty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /**A hierarchical array of the legacy categories to which this transaction belongs. For a full list of legacy categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

We recommend using the `personal_finance_category` for transaction categorization to obtain the best results.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_category: Option<Vec<String>>,
    /**The ID of the legacy category to which this transaction belongs. For a full list of legacy categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

We recommend using the `personal_finance_category` for transaction categorization to obtain the best results.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_category_id: Option<String>,
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
    pub personal_finance_category_icon_url: String,
    ///The phone number associated with the primary_counterparty in E. 164 format. If there is a location match (i.e. a street address is returned in the location object), the phone number will be location specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /**Insights relating to expenses and deposits that are predicted to occur on a scheduled basis, such as biweekly, monthly, or annually.

Common examples include loan payments, bill payments, subscriptions, and payroll income.

This is a beta field, available to all users.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
    ///The website associated with this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl std::fmt::Display for Enrichments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}