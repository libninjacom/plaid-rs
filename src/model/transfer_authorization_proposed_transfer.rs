use serde::{Serialize, Deserialize};
use super::{TransferCreditFundsSource, TransferUserInResponse};
///Details regarding the proposed transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAuthorizationProposedTransfer {
    ///The Plaid `account_id` for the account that will be debited or credited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, e.g. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ach_class: Option<String>,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00"). When calling `/transfer/authorization/create`, specify the maximum amount to authorize. When calling `/transfer/create`, specify the exact amount of the transfer, up to a maximum of the amount authorized. If this field is left blank when calling `/transfer/create`, the maximum amount authorized in the `authorization_id` will be sent.
    pub amount: String,
    pub credit_funds_source: TransferCreditFundsSource,
    ///The id of the associated funding account, available in the Plaid Dashboard. If present, this indicates which of your business checking accounts will be credited or debited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding_account_id: Option<String>,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: String,
    ///The network or rails used for the transfer.
    pub network: String,
    ///Plaid's unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///The Plaid client ID that is the originator of this transfer. Only present if created on behalf of another client as a [Platform customer](https://plaid.com/docs/transfer/application/#originators-vs-platforms).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_client_id: Option<String>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    #[serde(rename = "type")]
    pub type_: String,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
}
impl std::fmt::Display for TransferAuthorizationProposedTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}