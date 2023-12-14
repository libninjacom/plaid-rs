use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_application_list`].

On request success, this will return a [`ItemApplicationListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemApplicationListRequest {
    pub access_token: Option<String>,
}
impl ItemApplicationListRequest {}
impl FluentRequest<'_, ItemApplicationListRequest> {
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemApplicationListRequest> {
    type Output = httpclient::InMemoryResult<ItemApplicationListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/item/application/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}