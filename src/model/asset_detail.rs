use serde::{Serialize, Deserialize};
///Details about an asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetDetail {
    ///A unique alphanumeric string identifying an asset.
    #[serde(rename = "AssetAccountIdentifier")]
    pub asset_account_identifier: String,
    ///Account Report As of Date / Create Date. Format YYYY-MM-DD
    #[serde(rename = "AssetAsOfDate")]
    pub asset_as_of_date: String,
    ///Asset Account Available Balance.
    #[serde(rename = "AssetAvailableBalanceAmount")]
    pub asset_available_balance_amount: f64,
    ///A vendor created unique Identifier
    #[serde(rename = "AssetCurrentBalanceAmount")]
    pub asset_current_balance_amount: f64,
    ///The Number of days requested made to the Financial Institution. Example When looking for 3 months of data from the FI, pass in 90 days.
    #[serde(rename = "AssetDaysRequestedCount")]
    pub asset_days_requested_count: i64,
    ///A text description that further defines the Asset. This could be used to describe the shares associated with the stocks, bonds or mutual funds, retirement funds or business owned that the borrower has disclosed (named) as an asset.
    #[serde(rename = "AssetDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_description: Option<String>,
    ///Ownership type of the asset account.
    #[serde(rename = "AssetOwnershipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_ownership_type: Option<String>,
    ///A value from a MISMO prescribed list that specifies financial assets in a mortgage loan transaction. Assets may be either liquid or fixed and are associated with a corresponding asset amount.
    #[serde(rename = "AssetType")]
    pub asset_type: String,
    ///Additional Asset Decription some examples are Investment Tax-Deferred , Loan, 401K, 403B, Checking, Money Market, Credit Card,ROTH,529,Biller,ROLLOVER,CD,Savings,Investment Taxable, IRA, Mortgage, Line Of Credit.
    #[serde(rename = "AssetTypeAdditionalDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_type_additional_description: Option<String>,
    ///A vendor created unique Identifier.
    #[serde(rename = "AssetUniqueIdentifier")]
    pub asset_unique_identifier: String,
}
impl std::fmt::Display for AssetDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}