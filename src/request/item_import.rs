use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemImportRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub products: Vec<String>,
    pub user_auth: ItemImportRequestUserAuth,
    pub options: Option<ItemImportRequestOptions>,
}
impl<'a> ItemImportRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ItemImportResponse> {
        let mut r = self.http_client.client.post("/item/import");
        r = r.json(json!({ "products" : self.products }));
        r = r.json(json!({ "user_auth" : self.user_auth }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: ItemImportRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ItemImportRequest<'a> {
    type Output = httpclient::InMemoryResult<ItemImportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}