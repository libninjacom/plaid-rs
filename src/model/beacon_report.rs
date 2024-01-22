use serde::{Serialize, Deserialize};
use super::{BeaconAuditTrail, FraudAmount};
/**A Beacon Report describes the type of fraud committed by a user as well as the date the fraud was committed and the total amount of money lost due to the fraud incident.

This information is used to block similar fraud attempts on your platform as well as alert other companies who screen a user with matching identity information.
Other companies will not receive any new identity information, just what matched, plus information such as industry, type of fraud, and date of fraud.

You can manage your fraud reports by adding, deleting, or editing reports as you get additional information on fraudulent users.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReport {
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: BeaconAuditTrail,
    ///ID of the associated Beacon User.
    pub beacon_user_id: String,
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**The amount and currency of the fraud or attempted fraud.
`fraud_amount` should be omitted to indicate an unknown fraud amount.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraud_amount: Option<FraudAmount>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub fraud_date: chrono::NaiveDate,
    ///ID of the associated Beacon Report.
    pub id: String,
    /**The type of Beacon Report.

`first_party`: If this is the same individual as the one who submitted the KYC.

`stolen`: If this is a different individual from the one who submitted the KYC.

`synthetic`: If this is an individual using fabricated information.

`account_takeover`: If this individual's account was compromised.

`unknown`: If you aren't sure who committed the fraud.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BeaconReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}