use serde::{Serialize, Deserialize};
use super::RecipientBacs;
///Additional payment consent options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalPaymentInitiationConsentOptions {
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///The International Bank Account Number (IBAN) for the payer's account. Where possible, the end user will be able to set up payment consent using only the specified bank account if provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_refund_details: Option<bool>,
}
impl std::fmt::Display for ExternalPaymentInitiationConsentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}