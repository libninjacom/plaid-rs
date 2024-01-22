use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_transactions_get`].

On request success, this will return a [`ProcessorTransactionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTransactionsGetRequest {
    pub end_date: chrono::NaiveDate,
    pub options: Option<ProcessorTransactionsGetRequestOptions>,
    pub processor_token: String,
    pub start_date: chrono::NaiveDate,
}
impl ProcessorTransactionsGetRequest {}
impl FluentRequest<'_, ProcessorTransactionsGetRequest> {
    pub fn options(mut self, options: ProcessorTransactionsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTransactionsGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTransactionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/transactions/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "end_date" : self.params.end_date }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "processor_token" : self.params.processor_token }));
            r = r.json(json!({ "start_date" : self.params.start_date }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}