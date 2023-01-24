use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationRecipientGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub recipient_id: String,
}
impl<'a> PaymentInitiationRecipientGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationRecipientGetResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/recipient/get");
        r = r.json(json!({ "recipient_id" : self.recipient_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationRecipientGetRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationRecipientGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
