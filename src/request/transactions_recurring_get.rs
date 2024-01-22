use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_recurring_get`].

On request success, this will return a [`TransactionsRecurringGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequest {
    pub access_token: String,
    pub account_ids: Vec<String>,
    pub options: Option<TransactionsRecurringGetRequestOptions>,
}
impl TransactionsRecurringGetRequest {}
impl FluentRequest<'_, TransactionsRecurringGetRequest> {
    pub fn options(mut self, options: TransactionsRecurringGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsRecurringGetRequest> {
    type Output = httpclient::InMemoryResult<TransactionsRecurringGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transactions/recurring/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "account_ids" : self.params.account_ids }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}