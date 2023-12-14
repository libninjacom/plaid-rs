use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerOauthInstitutionsGetRequest {
    pub client_id: Option<String>,
    pub end_customer_client_id: String,
    pub secret: Option<String>,
}
impl PartnerCustomerOauthInstitutionsGetRequest {}
impl FluentRequest<'_, PartnerCustomerOauthInstitutionsGetRequest> {
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.params.client_id = Some(client_id.to_owned());
        self
    }
    pub fn secret(mut self, secret: &str) -> Self {
        self.params.secret = Some(secret.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PartnerCustomerOauthInstitutionsGetRequest> {
    type Output = httpclient::InMemoryResult<
        PartnerCustomerOAuthInstitutionsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/partner/customer/oauth_institutions/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}