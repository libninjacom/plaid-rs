use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsEnhanceRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub account_type: String,
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl<'a> TransactionsEnhanceRequest<'a> {
    pub async fn send(self) -> crate::Result<TransactionsEnhanceGetResponse> {
        let mut r = self
            .http_client
            .client
            .post("/beta/transactions/v1/enhance");
        r = r.json(json!({ "account_type" : self.account_type }));
        r = r.json(json!({ "transactions" : self.transactions }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsEnhanceRequest<'a> {
    type Output = crate::Result<TransactionsEnhanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
