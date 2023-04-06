use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsEnrichRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub account_type: String,
    pub options: Option<TransactionsEnrichRequestOptions>,
    pub transactions: Vec<ClientProvidedTransaction>,
}
impl<'a> TransactionsEnrichRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransactionsEnrichGetResponse> {
        let mut r = self.http_client.client.post("/transactions/enrich");
        r = r.json(json!({ "account_type" : self.account_type }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "transactions" : self.transactions }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: TransactionsEnrichRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsEnrichRequest<'a> {
    type Output = httpclient::InMemoryResult<TransactionsEnrichGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}