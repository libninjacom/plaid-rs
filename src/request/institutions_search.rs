use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::institutions_search`].

On request success, this will return a [`InstitutionsSearchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsSearchRequest {
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsSearchRequestOptions>,
    pub products: Option<Vec<String>>,
    pub query: String,
}
impl InstitutionsSearchRequest {}
impl FluentRequest<'_, InstitutionsSearchRequest> {
    pub fn options(mut self, options: InstitutionsSearchRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    pub fn products(
        mut self,
        products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .products = Some(
            products.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InstitutionsSearchRequest> {
    type Output = httpclient::InMemoryResult<InstitutionsSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/institutions/search";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}