use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_enrich`].

On request success, this will return a [`TransactionsEnrichResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsEnrichRequest {
    pub account_type: String,
    pub options: Option<TransactionsEnrichRequestOptions>,
    pub transactions: Vec<ClientProvidedTransaction>,
}
impl TransactionsEnrichRequest {}
impl FluentRequest<'_, TransactionsEnrichRequest> {
    pub fn options(mut self, options: TransactionsEnrichRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransactionsEnrichRequest> {
    type Output = httpclient::InMemoryResult<TransactionsEnrichResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transactions/enrich";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "account_type" : self.params.account_type }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "transactions" : self.params.transactions }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}