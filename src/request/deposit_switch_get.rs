use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DepositSwitchGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DepositSwitchGetResponse> {
        let mut r = self.http_client.client.post("/deposit_switch/get");
        r = r.json(json!({ "deposit_switch_id" : self.deposit_switch_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DepositSwitchGetRequest<'a> {
    type Output = httpclient::InMemoryResult<DepositSwitchGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
