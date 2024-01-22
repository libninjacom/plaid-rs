use serde::{Serialize, Deserialize};
///Taxpayer ID of the individual receiving the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxpayerId {
    ///ID mask; i.e. last 4 digits of the taxpayer ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_mask: Option<String>,
    ///Type of ID, e.g. 'SSN'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    ///Last 4 digits of unique number of ID.
    #[serde(rename = "last_4_digits")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last4_digits: Option<String>,
}
impl std::fmt::Display for TaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}