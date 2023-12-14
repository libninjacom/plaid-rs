use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::institutions_get`].

On request success, this will return a [`InstitutionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsGetRequest {
    pub count: i64,
    pub country_codes: Vec<String>,
    pub offset: i64,
    pub options: Option<InstitutionsGetRequestOptions>,
}
impl InstitutionsGetRequest {}
impl FluentRequest<'_, InstitutionsGetRequest> {
    pub fn options(mut self, options: InstitutionsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InstitutionsGetRequest> {
    type Output = httpclient::InMemoryResult<InstitutionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/institutions/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}