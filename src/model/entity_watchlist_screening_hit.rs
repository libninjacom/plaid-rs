use serde::{Serialize, Deserialize};
use super::{EntityScreeningHitAnalysis, EntityScreeningHitData};
///Data from a government watchlist that has been attached to the screening.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityWatchlistScreeningHit {
    ///Analysis information describing why a screening hit matched the provided entity information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<EntityScreeningHitAnalysis>,
    ///Information associated with the entity watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EntityScreeningHitData>,
    ///An ISO8601 formatted timestamp.
    pub first_active: chrono::DateTime<chrono::Utc>,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_since: Option<chrono::DateTime<chrono::Utc>>,
    ///ID of the associated entity screening hit.
    pub id: String,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive_since: Option<chrono::DateTime<chrono::Utc>>,
    /**Shorthand identifier for a specific screening list for entities.
 `AU_CON`: Australia Department of Foreign Affairs and Trade Consolidated List
 `CA_CON`: Government of Canada Consolidated List of Sanctions
 `EU_CON`: European External Action Service Consolidated List
 `IZ_SOE`: State Owned Enterprise List
 `IZ_UNC`: United Nations Consolidated Sanctions
 `IZ_WBK`: World Bank Listing of Ineligible Firms and Individuals
 `US_CAP`: US OFAC Correspondent Account or Payable-Through Account Sanctions
 `US_FSE`: US OFAC Foreign Sanctions Evaders
 `US_MBS`: US Non-SDN Menu-Based Sanctions
 `US_SDN`: US Specially Designated Nationals List
 `US_SSI`: US OFAC Sectoral Sanctions Identifications
 `US_CMC`: US OFAC Non-SDN Chinese Military-Industrial Complex List
 `US_UVL`: Bureau of Industry and Security Unverified List
 `UK_HMC`: UK HM Treasury Consolidated List*/
    pub list_code: String,
    ///A universal identifier for a watchlist individual that is stable across searches and updates.
    pub plaid_uid: String,
    ///The current state of review. All watchlist screening hits begin in a `pending_review` state but can be changed by creating a review. When a hit is in the `pending_review` state, it will always show the latest version of the watchlist data Plaid has available and be compared against the latest customer information saved in the watchlist screening. Once a hit has been marked as `confirmed` or `dismissed` it will no longer be updated so that the state is as it was when the review was first conducted.
    pub review_status: String,
    ///The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_uid: Option<String>,
}
impl std::fmt::Display for EntityWatchlistScreeningHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}