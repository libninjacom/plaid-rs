use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::wallet_create`].

On request success, this will return a [`WalletCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreateRequest {
    pub iso_currency_code: String,
}
impl WalletCreateRequest {}
impl FluentRequest<'_, WalletCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletCreateRequest> {
    type Output = httpclient::InMemoryResult<WalletCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/wallet/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}