use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::item_activity_list`].

On request success, this will return a [`ItemActivityListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemActivityListRequest {
    pub access_token: Option<String>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}
impl ItemActivityListRequest {}
impl FluentRequest<'_, ItemActivityListRequest> {
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemActivityListRequest> {
    type Output = httpclient::InMemoryResult<ItemActivityListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/activity/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(json!({ "access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}