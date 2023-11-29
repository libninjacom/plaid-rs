use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ItemImportRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub options: Option<ItemImportRequestOptions>,
    pub products: Vec<String>,
    pub user_auth: ItemImportRequestUserAuth,
}
impl<'a> ItemImportRequest<'a> {
    pub async fn send(self) -> crate::Result<ItemImportResponse> {
        let mut r = self.http_client.client.post("/item/import");
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "products" : self.products }));
        r = r.json(json!({ "user_auth" : self.user_auth }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: ItemImportRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ItemImportRequest<'a> {
    type Output = crate::Result<ItemImportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
