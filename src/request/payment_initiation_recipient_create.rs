use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateRequest {
    pub address: Option<PaymentInitiationAddress>,
    pub bacs: Option<RecipientBacsNullable>,
    pub iban: Option<String>,
    pub name: String,
}
impl PaymentInitiationRecipientCreateRequest {}
impl FluentRequest<'_, PaymentInitiationRecipientCreateRequest> {
    pub fn address(mut self, address: PaymentInitiationAddress) -> Self {
        self.params.address = Some(address);
        self
    }
    pub fn bacs(mut self, bacs: RecipientBacsNullable) -> Self {
        self.params.bacs = Some(bacs);
        self
    }
    pub fn iban(mut self, iban: &str) -> Self {
        self.params.iban = Some(iban.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationRecipientCreateRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationRecipientCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/payment_initiation/recipient/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}