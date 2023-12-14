use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::deposit_switch_get`].

On request success, this will return a [`DepositSwitchGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchGetRequest {
    pub deposit_switch_id: String,
}
impl DepositSwitchGetRequest {}
impl FluentRequest<'_, DepositSwitchGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DepositSwitchGetRequest> {
    type Output = httpclient::InMemoryResult<DepositSwitchGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/deposit_switch/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}