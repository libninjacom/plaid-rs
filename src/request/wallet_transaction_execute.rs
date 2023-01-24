use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletTransactionExecuteRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub idempotency_key: String,
    pub wallet_id: String,
    pub counterparty: WalletTransactionCounterparty,
    pub amount: WalletTransactionAmount,
    pub reference: String,
}
impl<'a> WalletTransactionExecuteRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WalletTransactionExecuteResponse> {
        let mut r = self.http_client.client.post("/wallet/transaction/execute");
        r = r.json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.json(json!({ "wallet_id" : self.wallet_id }));
        r = r.json(json!({ "counterparty" : self.counterparty }));
        r = r.json(json!({ "amount" : self.amount }));
        r = r.json(json!({ "reference" : self.reference }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
pub struct WalletTransactionExecuteRequired<'a> {
    pub idempotency_key: &'a str,
    pub wallet_id: &'a str,
    pub counterparty: WalletTransactionCounterparty,
    pub amount: WalletTransactionAmount,
    pub reference: &'a str,
}
impl<'a> WalletTransactionExecuteRequired<'a> {}
impl<'a> ::std::future::IntoFuture for WalletTransactionExecuteRequest<'a> {
    type Output = httpclient::InMemoryResult<WalletTransactionExecuteResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
