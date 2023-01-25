use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxTransferTestClockCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub virtual_time: String,
}
impl<'a> SandboxTransferTestClockCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SandboxTransferTestClockCreateResponse> {
        let mut r = self.http_client.client.post("/sandbox/transfer/test_clock/create");
        r = r.json(json!({ "virtual_time" : self.virtual_time }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for SandboxTransferTestClockCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<SandboxTransferTestClockCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}