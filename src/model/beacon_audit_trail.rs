use serde::{Serialize, Deserialize};
///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconAuditTrail {
    ///ID of the associated user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard_user_id: Option<String>,
    /**A type indicating what caused a resource to be changed or updated.


`dashboard` - The resource was created or updated by a member of your team via the Plaid dashboard.

`api` - The resource was created or updated via the Plaid API.

`system` - The resource was created or updated automatically by a part of the Plaid Beacon system. For example, if another business using Plaid Beacon created a fraud report that matched one of your users, your matching user's status would automatically be updated and the audit trail source would be `system`.

`bulk_import` - The resource was created or updated as part of a bulk import process. For example, if your company provided a CSV of user data as part of your initial onboarding, the audit trail source would be `bulk_import`.*/
    pub source: String,
}
impl std::fmt::Display for BeaconAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}