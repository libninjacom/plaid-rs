use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DepositSwitchTokenCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DepositSwitchTokenCreateResponse> {
        let mut r = self.http_client.client.post("/deposit_switch/token/create");
        r = r.json(json!({ "deposit_switch_id" : self.deposit_switch_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DepositSwitchTokenCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<DepositSwitchTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}