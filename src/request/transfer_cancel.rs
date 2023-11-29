use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferCancelRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub originator_client_id: Option<String>,
    pub transfer_id: String,
}
impl<'a> TransferCancelRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferCancelResponse> {
        let mut r = self.http_client.client.post("/transfer/cancel");
        if let Some(ref unwrapped) = self.originator_client_id {
            r = r.json(json!({ "originator_client_id" : unwrapped }));
        }
        r = r.json(json!({ "transfer_id" : self.transfer_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferCancelRequest<'a> {
    type Output = crate::Result<TransferCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
