use serde::{Serialize, Deserialize};
use super::{PaymentScheme, RecipientBacs};
///Additional payment options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentOptions {
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///The International Bank Account Number (IBAN) for the payer's account. Where possible, the end user will be able to send payments only from the specified bank account if provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_refund_details: Option<bool>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`LOCAL_DEFAULT`: The default payment scheme for the selected market and currency will be used.

`LOCAL_INSTANT`: The instant payment scheme for the selected market and currency will be used (if applicable). Fees may be applied by the institution. If the market does not support an Instant Scheme (e.g. Denmark), the default in the region will be used.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<PaymentScheme>,
}
impl std::fmt::Display for ExternalPaymentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}