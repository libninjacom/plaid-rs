use serde::{Serialize, Deserialize};
///Details about the status of the payroll item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayrollItemStatus {
    /**Denotes the processing status for the verification.

`UNKNOWN`: The processing status could not be determined.

`PROCESSING_COMPLETE`: The processing has completed and the user has approved for sharing. The data is available to be retrieved.

`PROCESSING`: The verification is still processing. The data is not available yet.

`FAILED`: The processing failed to complete successfully.

`APPROVAL_STATUS_PENDING`: The processing has completed but the user has not yet approved the sharing of the data.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<String>,
}
impl std::fmt::Display for PayrollItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}