use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::wallet_get`].

On request success, this will return a [`WalletGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletGetRequest {
    pub wallet_id: String,
}
impl WalletGetRequest {}
impl FluentRequest<'_, WalletGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletGetRequest> {
    type Output = httpclient::InMemoryResult<WalletGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "wallet_id" : self.params.wallet_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}