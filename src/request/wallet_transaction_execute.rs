use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletTransactionExecuteRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: String,
    pub reference: String,
    pub wallet_id: String,
}
impl<'a> WalletTransactionExecuteRequest<'a> {
    pub async fn send(self) -> crate::Result<WalletTransactionExecuteResponse> {
        let mut r = self.http_client.client.post("/wallet/transaction/execute");
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "counterparty" : self.counterparty }));
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "reference" : self.reference }));
        r = r.json(json!({ "wallet_id" : self.wallet_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
pub struct WalletTransactionExecuteRequired<'a> {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: &'a str,
    pub reference: &'a str,
    pub wallet_id: &'a str,
}
impl<'a> WalletTransactionExecuteRequired<'a> {}
impl<'a> ::std::future::IntoFuture for WalletTransactionExecuteRequest<'a> {
    type Output = crate::Result<WalletTransactionExecuteResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
