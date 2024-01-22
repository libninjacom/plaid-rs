use serde::{Serialize, Deserialize};
///Metadata specifically related to valid Payment Initiation standing order configurations for the institution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationStandingOrderMetadata {
    ///Indicates whether the institution supports closed-ended standing orders by providing an end date.
    pub supports_standing_order_end_date: bool,
    ///This is only applicable to `MONTHLY` standing orders. Indicates whether the institution supports negative integers (-1 to -5) for setting up a `MONTHLY` standing order relative to the end of the month.
    pub supports_standing_order_negative_execution_days: bool,
    ///A list of the valid standing order intervals supported by the institution.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub valid_standing_order_intervals: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationStandingOrderMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}