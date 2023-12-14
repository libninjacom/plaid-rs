use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/item/activity/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}