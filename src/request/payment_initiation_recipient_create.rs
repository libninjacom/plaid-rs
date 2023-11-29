use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationRecipientCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub address: Option<PaymentInitiationAddress>,
    pub bacs: Option<RecipientBacsNullable>,
    pub iban: Option<String>,
    pub name: String,
}
impl<'a> PaymentInitiationRecipientCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<PaymentInitiationRecipientCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/payment_initiation/recipient/create");
        if let Some(ref unwrapped) = self.address {
            r = r.json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bacs {
            r = r.json(json!({ "bacs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iban {
            r = r.json(json!({ "iban" : unwrapped }));
        }
        r = r.json(json!({ "name" : self.name }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn address(mut self, address: PaymentInitiationAddress) -> Self {
        self.address = Some(address);
        self
    }
    pub fn bacs(mut self, bacs: RecipientBacsNullable) -> Self {
        self.bacs = Some(bacs);
        self
    }
    pub fn iban(mut self, iban: &str) -> Self {
        self.iban = Some(iban.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationRecipientCreateRequest<'a> {
    type Output = crate::Result<PaymentInitiationRecipientCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
