use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_user_update`].

On request success, this will return a [`BeaconUserUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserUpdateRequest {
    pub beacon_user_id: String,
    pub user: BeaconUserUpdateRequestData,
}
impl BeaconUserUpdateRequest {}
impl FluentRequest<'_, BeaconUserUpdateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserUpdateRequest> {
    type Output = httpclient::InMemoryResult<BeaconUserUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/update";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "beacon_user_id" : self.params.beacon_user_id }));
            r = r.json(json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}