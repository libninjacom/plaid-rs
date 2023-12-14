use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::dashboard_user_list`].

On request success, this will return a [`DashboardUserListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserListRequest {
    pub cursor: Option<String>,
}
impl DashboardUserListRequest {}
impl FluentRequest<'_, DashboardUserListRequest> {
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DashboardUserListRequest> {
    type Output = httpclient::InMemoryResult<DashboardUserListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/dashboard_user/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}