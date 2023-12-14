use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/user/update";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}