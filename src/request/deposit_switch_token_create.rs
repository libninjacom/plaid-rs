use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::deposit_switch_token_create`].

On request success, this will return a [`DepositSwitchTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateRequest {
    pub deposit_switch_id: String,
}
impl DepositSwitchTokenCreateRequest {}
impl FluentRequest<'_, DepositSwitchTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DepositSwitchTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<DepositSwitchTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/deposit_switch/token/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "deposit_switch_id" : self.params.deposit_switch_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}