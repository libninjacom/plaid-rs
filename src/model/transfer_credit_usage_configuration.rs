use serde::{Serialize, Deserialize};
///Specifies the originator's expected usage of credits. For all dollar amounts, use a decimal string with two digits of precision e.g. "10.00". This field is required if the originator is expected to process credit transfers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferCreditUsageConfiguration {
    ///The originator’s expected average amount per credit.
    pub expected_average_amount: String,
    ///The originator's expected transfer frequency.
    pub expected_frequency: String,
    ///The originator’s expected highest amount for a single credit transfer.
    pub expected_highest_amount: String,
    ///The originator’s monthly expected ACH credit processing amount for the next 6-12 months.
    pub expected_monthly_amount: String,
    /**Specifies the expected use cases for the originator’s credit transfers. This should be a list that contains one or more of the following codes:

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, e.g. bill payment

`"web"` - A credit Entry initiated by or on behalf of a holder of a Consumer Account that is intended for a Consumer Account of a Receiver*/
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sec_codes: Vec<String>,
}
impl std::fmt::Display for TransferCreditUsageConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}