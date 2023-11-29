use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CategoriesGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
}
impl<'a> CategoriesGetRequest<'a> {
    pub async fn send(self) -> crate::Result<CategoriesGetResponse> {
        let mut r = self.http_client.client.post("/categories/get");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for CategoriesGetRequest<'a> {
    type Output = crate::Result<CategoriesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
