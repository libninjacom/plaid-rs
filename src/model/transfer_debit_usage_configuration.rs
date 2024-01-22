use serde::{Serialize, Deserialize};
///Specifies the originator's expected usage of debits. For all dollar amounts, use a decimal string with two digits of precision e.g. "10.00". This field is required if the originator is expected to process debit transfers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferDebitUsageConfiguration {
    ///The originator’s expected average amount per debit.
    pub expected_average_amount: String,
    ///The originator's expected transfer frequency.
    pub expected_frequency: String,
    ///The originator’s expected highest amount for a single debit transfer.
    pub expected_highest_amount: String,
    ///The originator’s monthly expected ACH debit processing amount for the next 6-12 months.
    pub expected_monthly_amount: String,
    /**Specifies the expected use cases for the originator’s debit transfers. This should be a list that contains one or more of the following codes:

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sec_codes: Vec<String>,
}
impl std::fmt::Display for TransferDebitUsageConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}