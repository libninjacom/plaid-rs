use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_user_create`].

On request success, this will return a [`BeaconUserCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserCreateRequest {
    pub client_user_id: String,
    pub program_id: String,
    pub user: BeaconUserRequestData,
}
impl BeaconUserCreateRequest {}
impl FluentRequest<'_, BeaconUserCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserCreateRequest> {
    type Output = httpclient::InMemoryResult<BeaconUserCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/beacon/user/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}