use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_transactions_refresh`].

On request success, this will return a [`ProcessorTransactionsRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTransactionsRefreshRequest {
    pub processor_token: String,
}
impl ProcessorTransactionsRefreshRequest {}
impl FluentRequest<'_, ProcessorTransactionsRefreshRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTransactionsRefreshRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTransactionsRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/transactions/refresh";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}