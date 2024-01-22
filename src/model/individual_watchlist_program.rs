use serde::{Serialize, Deserialize};
use super::WatchlistScreeningAuditTrail;
///A program that configures the active lists, search parameters, and other behavior for initial and ongoing screening of individuals.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndividualWatchlistProgram {
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///ID of the associated program.
    pub id: String,
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: bool,
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    ///Watchlists enabled for the associated program
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lists_enabled: Vec<String>,
    ///A name for the program to define its purpose. For example, "High Risk Individuals", "US Cardholders", or "Applicants".
    pub name: String,
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: String,
}
impl std::fmt::Display for IndividualWatchlistProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}