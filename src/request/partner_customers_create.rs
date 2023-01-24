use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PartnerCustomersCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub company_name: String,
    pub is_diligence_attested: bool,
    pub products: Vec<String>,
    pub create_link_customization: Option<bool>,
}
impl<'a> PartnerCustomersCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PartnerCustomersCreateResponse> {
        let mut r = self.http_client.client.post("/beta/partner/v1/customers/create");
        r = r.json(json!({ "company_name" : self.company_name }));
        r = r.json(json!({ "is_diligence_attested" : self.is_diligence_attested }));
        r = r.json(json!({ "products" : self.products }));
        if let Some(ref unwrapped) = self.create_link_customization {
            r = r.json(json!({ "create_link_customization" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn create_link_customization(mut self, create_link_customization: bool) -> Self {
        self.create_link_customization = Some(create_link_customization);
        self
    }
}
impl<'a> ::std::future::IntoFuture for PartnerCustomersCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<PartnerCustomersCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
