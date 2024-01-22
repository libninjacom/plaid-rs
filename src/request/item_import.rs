use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_import`].

On request success, this will return a [`ItemImportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemImportRequest {
    pub options: Option<ItemImportRequestOptions>,
    pub products: Vec<String>,
    pub user_auth: ItemImportRequestUserAuth,
}
impl ItemImportRequest {}
impl FluentRequest<'_, ItemImportRequest> {
    pub fn options(mut self, options: ItemImportRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemImportRequest> {
    type Output = httpclient::InMemoryResult<ItemImportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/import";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "products" : self.params.products }));
            r = r.json(json!({ "user_auth" : self.params.user_auth }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}