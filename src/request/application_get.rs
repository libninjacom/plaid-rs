use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::application_get`].

On request success, this will return a [`ApplicationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationGetRequest {
    pub application_id: String,
}
impl ApplicationGetRequest {}
impl FluentRequest<'_, ApplicationGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ApplicationGetRequest> {
    type Output = httpclient::InMemoryResult<ApplicationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/application/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}