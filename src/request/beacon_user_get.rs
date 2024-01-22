use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_user_get`].

On request success, this will return a [`BeaconUserGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserGetRequest {
    pub beacon_user_id: String,
}
impl BeaconUserGetRequest {}
impl FluentRequest<'_, BeaconUserGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserGetRequest> {
    type Output = httpclient::InMemoryResult<BeaconUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "beacon_user_id" : self.params.beacon_user_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}