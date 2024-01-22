use serde::{Serialize, Deserialize};
use super::{ScreeningHitAnalysis, ScreeningHitData};
///Data from a government watchlist or PEP list that has been attached to the screening.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningHit {
    ///Analysis information describing why a screening hit matched the provided user information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ScreeningHitAnalysis>,
    ///Information associated with the watchlist hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ScreeningHitData>,
    ///An ISO8601 formatted timestamp.
    pub first_active: chrono::DateTime<chrono::Utc>,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_since: Option<chrono::DateTime<chrono::Utc>>,
    ///ID of the associated screening hit.
    pub id: String,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive_since: Option<chrono::DateTime<chrono::Utc>>,
    /**Shorthand identifier for a specific screening list for individuals.
 `AU_CON`: Australia Department of Foreign Affairs and Trade Consolidated List
 `CA_CON`: Government of Canada Consolidated List of Sanctions
 `EU_CON`: European External Action Service Consolidated List
 `IZ_CIA`: CIA List of Chiefs of State and Cabinet Members
 `IZ_IPL`: Interpol Red Notices for Wanted Persons List
 `IZ_PEP`: Politically Exposed Persons List
 `IZ_UNC`: United Nations Consolidated Sanctions
 `IZ_WBK`: World Bank Listing of Ineligible Firms and Individuals
 `UK_HMC`: UK HM Treasury Consolidated List
 `US_DPL`: Bureau of Industry and Security Denied Persons List
 `US_DTC`: US Department of State AECA Debarred
 `US_FBI`: US Department of Justice FBI Wanted List
 `US_FSE`: US OFAC Foreign Sanctions Evaders
 `US_ISN`: US Department of State Nonproliferation Sanctions
 `US_PLC`: US OFAC Palestinian Legislative Council
 `US_SDN`: US OFAC Specially Designated Nationals List
 `US_SSI`: US OFAC Sectoral Sanctions Identifications
 `SG_SOF`: Government of Singapore Terrorists and Terrorist Entities
 `TR_TWL`: Government of Turkey Terrorist Wanted List
 `TR_DFD`: Government of Turkey Domestic Freezing Decisions
 `TR_FOR`: Government of Turkey Foreign Freezing Requests
 `TR_WMD`: Government of Turkey Weapons of Mass Destruction
 `TR_CMB`: Government of Turkey Capital Markets Board*/
    pub list_code: String,
    ///A universal identifier for a watchlist individual that is stable across searches and updates.
    pub plaid_uid: String,
    ///The current state of review. All watchlist screening hits begin in a `pending_review` state but can be changed by creating a review. When a hit is in the `pending_review` state, it will always show the latest version of the watchlist data Plaid has available and be compared against the latest customer information saved in the watchlist screening. Once a hit has been marked as `confirmed` or `dismissed` it will no longer be updated so that the state is as it was when the review was first conducted.
    pub review_status: String,
    ///The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_uid: Option<String>,
}
impl std::fmt::Display for WatchlistScreeningHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}