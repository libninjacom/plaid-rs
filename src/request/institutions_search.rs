use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct InstitutionsSearchRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsSearchRequestOptions>,
    pub products: Option<Vec<String>>,
    pub query: String,
}
impl<'a> InstitutionsSearchRequest<'a> {
    pub async fn send(self) -> crate::Result<InstitutionsSearchResponse> {
        let mut r = self.http_client.client.post("/institutions/search");
        r = r.json(json!({ "country_codes" : self.country_codes }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.products {
            r = r.json(json!({ "products" : unwrapped }));
        }
        r = r.json(json!({ "query" : self.query }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: InstitutionsSearchRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn products(mut self, products: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.products = Some(
            products
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for InstitutionsSearchRequest<'a> {
    type Output = crate::Result<InstitutionsSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
