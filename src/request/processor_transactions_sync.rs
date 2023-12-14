use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_transactions_sync`].

On request success, this will return a [`ProcessorTransactionsSyncResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTransactionsSyncRequest {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub options: Option<TransactionsSyncRequestOptions>,
    pub processor_token: String,
}
impl ProcessorTransactionsSyncRequest {}
impl FluentRequest<'_, ProcessorTransactionsSyncRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    pub fn options(mut self, options: TransactionsSyncRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTransactionsSyncRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTransactionsSyncResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/transactions/sync";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}