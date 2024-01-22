use serde::{Serialize, Deserialize};
use super::{
    PersonalFinanceCategory, TransactionBase, TransactionCode, TransactionCounterparty,
};
///A representation of a transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    ///A representation of a transaction
    #[serde(flatten)]
    pub transaction_base: TransactionBase,
    ///The date that the transaction was authorized. For posted transactions, the `date` field will indicate the posted date, but `authorized_date` will indicate the day the transaction was authorized by the financial institution. If presenting transactions to the user in a UI, the `authorized_date`, when available, is generally preferable to use over the `date` field for posted transactions, as it will generally represent the date the user actually made the transaction. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_date: Option<chrono::NaiveDate>,
    /**Date and time when a transaction was authorized in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ). For posted transactions, the `datetime` field will indicate the posted date, but `authorized_datetime` will indicate the day the transaction was authorized by the financial institution. If presenting transactions to the user in a UI, the `authorized_datetime`, when available, is generally preferable to use over the `datetime` field for posted transactions, as it will generally represent the date the user actually made the transaction.

This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_datetime: Option<chrono::DateTime<chrono::Utc>>,
    ///The counterparties present in the transaction. Counterparties, such as the merchant or the financial institution, are extracted by Plaid from the raw description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<TransactionCounterparty>>,
    /**Date and time when a transaction was posted in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ). For the date that the transaction was initiated, rather than posted, see the `authorized_datetime` field.

This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00). This field is only populated in API version 2019-05-29 and later.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime: Option<chrono::DateTime<chrono::Utc>>,
    ///A unique, stable, Plaid-generated ID that maps to the merchant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_entity_id: Option<String>,
    /**The channel used to make a payment.
`online:` transactions that took place online.

`in store:` transactions that were made at a physical location.

`other:` transactions that relate to banks, e.g. fees or deposits.

This field replaces the `transaction_type` field.*/
    pub payment_channel: String,
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy CSV file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories. If you are migrating to personal finance categories from the legacy categories, also refer to the [`migration guide`](https://plaid.com/docs/transactions/pfc-migration/).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    ///The URL of an icon associated with the primary personal finance category. The icon will always be 100×100 pixel PNG file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_finance_category_icon_url: Option<String>,
    /**An identifier classifying the transaction type.

This field is only populated for European institutions. For institutions in the US and Canada, this field is set to `null`.

`adjustment:` Bank adjustment

`atm:` Cash deposit or withdrawal via an automated teller machine

`bank charge:` Charge or fee levied by the institution

`bill payment`: Payment of a bill

`cash:` Cash deposit or withdrawal

`cashback:` Cash withdrawal while making a debit card purchase

`cheque:` Document ordering the payment of money to another person or organization

`direct debit:` Automatic withdrawal of funds initiated by a third party at a regular interval

`interest:` Interest earned or incurred

`purchase:` Purchase made with a debit or credit card

`standing order:` Payment instructed by the account holder to a third party at a regular interval

`transfer:` Transfer of money between accounts*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<TransactionCode>,
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Transaction {
    type Target = TransactionBase;
    fn deref(&self) -> &Self::Target {
        &self.transaction_base
    }
}
impl std::ops::DerefMut for Transaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_base
    }
}