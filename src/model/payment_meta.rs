use serde::{Serialize, Deserialize};
/**Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/sync` or `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMeta {
    ///The party initiating a wire transfer. Will be `null` if the transaction is not a wire transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by_order_of: Option<String>,
    ///For transfers, the party that is receiving the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payee: Option<String>,
    ///For transfers, the party that is paying the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer: Option<String>,
    ///The type of transfer, e.g. 'ACH'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    ///The name of the payment processor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
    ///The ACH PPD ID for the payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ppd_id: Option<String>,
    ///The payer-supplied description of the transfer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///The transaction reference number supplied by the financial institution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
}
impl std::fmt::Display for PaymentMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}