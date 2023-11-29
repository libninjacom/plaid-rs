use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationRecipientListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
}
impl<'a> PaymentInitiationRecipientListRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationRecipientListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/recipient/list");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationRecipientListRequest<'a> {
    type Output = crate::Result<PaymentInitiationRecipientListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
