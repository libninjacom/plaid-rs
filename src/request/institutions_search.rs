use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct InstitutionsSearchRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub query: String,
    pub products: Option<Vec<String>>,
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsSearchRequestOptions>,
}
impl<'a> InstitutionsSearchRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<InstitutionsSearchResponse> {
        let mut r = self.http_client.client.post("/institutions/search");
        r = r.json(json!({ "query" : self.query }));
        if let Some(ref unwrapped) = self.products {
            r = r.json(json!({ "products" : unwrapped }));
        }
        r = r.json(json!({ "country_codes" : self.country_codes }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn products(
        mut self,
        products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .products = Some(
            products.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn options(mut self, options: InstitutionsSearchRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for InstitutionsSearchRequest<'a> {
    type Output = httpclient::InMemoryResult<InstitutionsSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}