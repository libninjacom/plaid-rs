
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTransactionDetail {
    #[serde(rename = "AssetTransactionAmount")]
    pub asset_transaction_amount: f64,
    #[serde(rename = "AssetTransactionCategoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_transaction_category_type: Option<String>,
    #[serde(rename = "AssetTransactionDate")]
    pub asset_transaction_date: chrono::NaiveDate,
    #[serde(rename = "AssetTransactionPaidByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_transaction_paid_by_name: Option<String>,
    #[serde(rename = "AssetTransactionPostDate")]
    pub asset_transaction_post_date: chrono::NaiveDate,
    #[serde(rename = "AssetTransactionType")]
    pub asset_transaction_type: String,
    #[serde(rename = "AssetTransactionTypeAdditionalDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_transaction_type_additional_description: Option<String>,
    #[serde(rename = "AssetTransactionUniqueIdentifier")]
    pub asset_transaction_unique_identifier: String,
    #[serde(rename = "FinancialInstitutionTransactionIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_institution_transaction_identifier: Option<String>,
}
impl std::fmt::Display for AssetTransactionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}