use serde::{Serialize, Deserialize};
///Data about military info in the income verification precheck.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    /**If the user is currently serving in the US military, the branch of the military in which they are serving
Valid values: 'AIR FORCE', 'ARMY', 'COAST GUARD', 'MARINES', 'NAVY', 'UNKNOWN'*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    ///Is the user currently active duty in the US military
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active_duty: Option<bool>,
}
impl std::fmt::Display for IncomeVerificationPrecheckMilitaryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}