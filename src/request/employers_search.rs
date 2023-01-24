use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct EmployersSearchRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub query: String,
    pub products: Vec<String>,
}
impl<'a> EmployersSearchRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<EmployersSearchResponse> {
        let mut r = self.http_client.client.post("/employers/search");
        r = r.json(json!({ "query" : self.query }));
        r = r.json(json!({ "products" : self.products }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for EmployersSearchRequest<'a> {
    type Output = httpclient::InMemoryResult<EmployersSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
