use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxTransferTestClockAdvanceRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub new_virtual_time: chrono::DateTime<chrono::Utc>,
    pub test_clock_id: String,
}
impl<'a> SandboxTransferTestClockAdvanceRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxTransferTestClockAdvanceResponse> {
        let mut r = self
            .http_client
            .client
            .post("/sandbox/transfer/test_clock/advance");
        r = r.json(json!({ "new_virtual_time" : self.new_virtual_time }));
        r = r.json(json!({ "test_clock_id" : self.test_clock_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for SandboxTransferTestClockAdvanceRequest<'a> {
    type Output = crate::Result<SandboxTransferTestClockAdvanceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
