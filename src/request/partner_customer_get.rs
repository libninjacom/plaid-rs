use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::partner_customer_get`].

On request success, this will return a [`PartnerCustomerGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerGetRequest {
    pub client_id: Option<String>,
    pub end_customer_client_id: String,
    pub secret: Option<String>,
}
impl PartnerCustomerGetRequest {}
impl FluentRequest<'_, PartnerCustomerGetRequest> {
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.params.client_id = Some(client_id.to_owned());
        self
    }
    pub fn secret(mut self, secret: &str) -> Self {
        self.params.secret = Some(secret.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PartnerCustomerGetRequest> {
    type Output = httpclient::InMemoryResult<PartnerCustomerGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/partner/customer/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.client_id {
                r = r.json(json!({ "client_id" : unwrapped }));
            }
            r = r
                .json(
                    json!(
                        { "end_customer_client_id" : self.params.end_customer_client_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.secret {
                r = r.json(json!({ "secret" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}