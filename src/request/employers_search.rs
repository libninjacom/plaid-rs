use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct EmployersSearchRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub products: Vec<String>,
    pub query: String,
}
impl<'a> EmployersSearchRequest<'a> {
    pub async fn send(self) -> crate::Result<EmployersSearchResponse> {
        let mut r = self.http_client.client.post("/employers/search");
        r = r.json(json!({ "products" : self.products }));
        r = r.json(json!({ "query" : self.query }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for EmployersSearchRequest<'a> {
    type Output = crate::Result<EmployersSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
