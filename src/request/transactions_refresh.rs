use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_refresh`].

On request success, this will return a [`TransactionsRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRefreshRequest {
    pub access_token: String,
}
impl TransactionsRefreshRequest {}
impl FluentRequest<'_, TransactionsRefreshRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransactionsRefreshRequest> {
    type Output = httpclient::InMemoryResult<TransactionsRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transactions/refresh";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}