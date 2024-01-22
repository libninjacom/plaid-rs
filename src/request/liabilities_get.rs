use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::liabilities_get`].

On request success, this will return a [`LiabilitiesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilitiesGetRequest {
    pub access_token: String,
    pub options: Option<LiabilitiesGetRequestOptions>,
}
impl LiabilitiesGetRequest {}
impl FluentRequest<'_, LiabilitiesGetRequest> {
    pub fn options(mut self, options: LiabilitiesGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LiabilitiesGetRequest> {
    type Output = httpclient::InMemoryResult<LiabilitiesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/liabilities/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}