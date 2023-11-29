use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxTransferTestClockCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub virtual_time: chrono::DateTime<chrono::Utc>,
}
impl<'a> SandboxTransferTestClockCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<SandboxTransferTestClockCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/sandbox/transfer/test_clock/create");
        r = r.json(json!({ "virtual_time" : self.virtual_time }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for SandboxTransferTestClockCreateRequest<'a> {
    type Output = crate::Result<SandboxTransferTestClockCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
