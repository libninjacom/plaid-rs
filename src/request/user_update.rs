use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::user_update`].

On request success, this will return a [`UserUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub consumer_report_user_identity: Option<ConsumerReportUserIdentity>,
    pub user_token: String,
}
impl UserUpdateRequest {}
impl FluentRequest<'_, UserUpdateRequest> {
    pub fn consumer_report_user_identity(
        mut self,
        consumer_report_user_identity: ConsumerReportUserIdentity,
    ) -> Self {
        self.params.consumer_report_user_identity = Some(consumer_report_user_identity);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UserUpdateRequest> {
    type Output = httpclient::InMemoryResult<UserUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/update";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.consumer_report_user_identity {
                r = r.json(json!({ "consumer_report_user_identity" : unwrapped }));
            }
            r = r.json(json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}