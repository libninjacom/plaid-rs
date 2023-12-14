use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::investments_auth_get`].

On request success, this will return a [`InvestmentsAuthGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsAuthGetRequest {
    pub access_token: String,
    pub options: Option<InvestmentsAuthGetRequestOptions>,
}
impl InvestmentsAuthGetRequest {}
impl FluentRequest<'_, InvestmentsAuthGetRequest> {
    pub fn options(mut self, options: InvestmentsAuthGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InvestmentsAuthGetRequest> {
    type Output = httpclient::InMemoryResult<InvestmentsAuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/investments/auth/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}