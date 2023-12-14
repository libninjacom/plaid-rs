use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_user_review`].

On request success, this will return a [`BeaconUserGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserReviewRequest {
    pub beacon_user_id: String,
    pub status: String,
}
impl BeaconUserReviewRequest {}
impl FluentRequest<'_, BeaconUserReviewRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserReviewRequest> {
    type Output = httpclient::InMemoryResult<BeaconUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/beacon/user/review";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}