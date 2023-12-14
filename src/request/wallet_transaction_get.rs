use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::wallet_transaction_get`].

On request success, this will return a [`WalletTransactionGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionGetRequest {
    pub transaction_id: String,
}
impl WalletTransactionGetRequest {}
impl FluentRequest<'_, WalletTransactionGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletTransactionGetRequest> {
    type Output = httpclient::InMemoryResult<WalletTransactionGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/wallet/transaction/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}