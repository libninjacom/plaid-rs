
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetTransactionDetail {
    #[serde(rename = "AssetTransactionAmount")]
    pub asset_transaction_amount: f64,
    #[serde(rename = "AssetTransactionCategoryType")]
    pub asset_transaction_category_type: Option<String>,
    #[serde(rename = "AssetTransactionDate")]
    pub asset_transaction_date: String,
    #[serde(rename = "AssetTransactionPaidByName")]
    pub asset_transaction_paid_by_name: Option<String>,
    #[serde(rename = "AssetTransactionPostDate")]
    pub asset_transaction_post_date: String,
    #[serde(rename = "AssetTransactionType")]
    pub asset_transaction_type: String,
    #[serde(rename = "AssetTransactionTypeAdditionalDescription")]
    pub asset_transaction_type_additional_description: Option<String>,
    #[serde(rename = "AssetTransactionUniqueIdentifier")]
    pub asset_transaction_unique_identifier: String,
    #[serde(rename = "FinancialInstitutionTransactionIdentifier")]
    pub financial_institution_transaction_identifier: Option<String>,
}
impl std::fmt::Display for AssetTransactionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}