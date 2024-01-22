use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_enhance`].

On request success, this will return a [`TransactionsEnhanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsEnhanceRequest {
    pub account_type: String,
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl TransactionsEnhanceRequest {}
impl FluentRequest<'_, TransactionsEnhanceRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransactionsEnhanceRequest> {
    type Output = httpclient::InMemoryResult<TransactionsEnhanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/v1/enhance";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "account_type" : self.params.account_type }));
            r = r.json(json!({ "transactions" : self.params.transactions }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}