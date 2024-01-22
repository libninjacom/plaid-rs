use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_duplicate_get`].

On request success, this will return a [`BeaconDuplicateGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconDuplicateGetRequest {
    pub beacon_duplicate_id: String,
}
impl BeaconDuplicateGetRequest {}
impl FluentRequest<'_, BeaconDuplicateGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconDuplicateGetRequest> {
    type Output = httpclient::InMemoryResult<BeaconDuplicateGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/duplicate/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!({ "beacon_duplicate_id" : self.params.beacon_duplicate_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}