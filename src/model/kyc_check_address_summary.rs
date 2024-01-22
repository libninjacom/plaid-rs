use serde::{Serialize, Deserialize};
///Result summary object specifying how the `address` field matched.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KycCheckAddressSummary {
    ///Field describing whether the associated address is a post office box. Will be `yes` when a P.O. box is detected, `no` when Plaid confirmed the address is not a P.O. box, and `no_data` when Plaid was not able to determine if the address is a P.O. box.
    pub po_box: String,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
    /**Field describing whether the associated address is being used for commercial or residential purposes.

Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for KycCheckAddressSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}