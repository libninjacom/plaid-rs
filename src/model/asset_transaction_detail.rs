use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetTransactionDetail {
    ///Asset Transaction Amount.
    #[serde(rename = "AssetTransactionAmount")]
    pub asset_transaction_amount: f64,
    ///Asset Transaction Category Type Enumerated derived by Vendor.
    #[serde(rename = "AssetTransactionCategoryType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_transaction_category_type: Option<String>,
    ///Asset Transaction Date.
    #[serde(rename = "AssetTransactionDate")]
    pub asset_transaction_date: chrono::NaiveDate,
    ///Populate with who did the transaction.
    #[serde(rename = "AssetTransactionPaidByName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_transaction_paid_by_name: Option<String>,
    ///Asset Transaction Post Date.
    #[serde(rename = "AssetTransactionPostDate")]
    pub asset_transaction_post_date: chrono::NaiveDate,
    ///Asset Transaction Type.
    #[serde(rename = "AssetTransactionType")]
    pub asset_transaction_type: String,
    ///FI Provided - examples are atm, cash, check, credit, debit, deposit, directDebit, directDeposit, dividend, fee, interest, other, payment, pointOfSale, repeatPayment, serviceCharge, transfer.
    #[serde(rename = "AssetTransactionTypeAdditionalDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_transaction_type_additional_description: Option<String>,
    ///A vendor created unique Identifier.
    #[serde(rename = "AssetTransactionUniqueIdentifier")]
    pub asset_transaction_unique_identifier: String,
    ///FI provided Transaction Identifier.
    #[serde(rename = "FinancialInstitutionTransactionIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub financial_institution_transaction_identifier: Option<String>,
}
impl std::fmt::Display for AssetTransactionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}