use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::base_report_get`].

On request success, this will return a [`BaseReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportGetRequest {
    pub user_token: String,
}
impl BaseReportGetRequest {}
impl FluentRequest<'_, BaseReportGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BaseReportGetRequest> {
    type Output = httpclient::InMemoryResult<BaseReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/base_report/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}