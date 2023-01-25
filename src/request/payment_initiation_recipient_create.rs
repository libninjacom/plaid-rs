use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationRecipientCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub name: String,
    pub iban: Option<String>,
    pub bacs: Option<RecipientBacsNullable>,
    pub address: Option<PaymentInitiationAddress>,
}
impl<'a> PaymentInitiationRecipientCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationRecipientCreateResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/recipient/create");
        r = r.json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.iban {
            r = r.json(json!({ "iban" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bacs {
            r = r.json(json!({ "bacs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address {
            r = r.json(json!({ "address" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn iban(mut self, iban: &str) -> Self {
        self.iban = Some(iban.to_owned());
        self
    }
    pub fn bacs(mut self, bacs: RecipientBacsNullable) -> Self {
        self.bacs = Some(bacs);
        self
    }
    pub fn address(mut self, address: PaymentInitiationAddress) -> Self {
        self.address = Some(address);
        self
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationRecipientCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationRecipientCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}