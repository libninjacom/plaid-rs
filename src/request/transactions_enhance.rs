use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/beta/transactions/v1/enhance";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}