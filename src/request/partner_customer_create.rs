use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PartnerCustomerCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_id: Option<String>,
    pub secret: Option<String>,
    pub company_name: String,
    pub is_diligence_attested: bool,
    pub products: Vec<String>,
    pub create_link_customization: Option<bool>,
    pub logo: Option<String>,
    pub legal_entity_name: String,
    pub website: String,
    pub application_name: String,
    pub technical_contact: Option<PartnerEndCustomerTechnicalContact>,
    pub billing_contact: Option<PartnerEndCustomerBillingContact>,
    pub customer_support_info: Option<PartnerEndCustomerCustomerSupportInfo>,
    pub address: PartnerEndCustomerAddress,
    pub is_bank_addendum_completed: bool,
    pub assets_under_management: Option<PartnerEndCustomerAssetsUnderManagement>,
}
impl<'a> PartnerCustomerCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PartnerCustomerCreateResponse> {
        let mut r = self.http_client.client.post("/partner/customer/create");
        if let Some(ref unwrapped) = self.client_id {
            r = r.json(json!({ "client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.secret {
            r = r.json(json!({ "secret" : unwrapped }));
        }
        r = r.json(json!({ "company_name" : self.company_name }));
        r = r.json(json!({ "is_diligence_attested" : self.is_diligence_attested }));
        r = r.json(json!({ "products" : self.products }));
        if let Some(ref unwrapped) = self.create_link_customization {
            r = r.json(json!({ "create_link_customization" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.logo {
            r = r.json(json!({ "logo" : unwrapped }));
        }
        r = r.json(json!({ "legal_entity_name" : self.legal_entity_name }));
        r = r.json(json!({ "website" : self.website }));
        r = r.json(json!({ "application_name" : self.application_name }));
        if let Some(ref unwrapped) = self.technical_contact {
            r = r.json(json!({ "technical_contact" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billing_contact {
            r = r.json(json!({ "billing_contact" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.customer_support_info {
            r = r.json(json!({ "customer_support_info" : unwrapped }));
        }
        r = r.json(json!({ "address" : self.address }));
        r = r
            .json(
                json!({ "is_bank_addendum_completed" : self.is_bank_addendum_completed }),
            );
        if let Some(ref unwrapped) = self.assets_under_management {
            r = r.json(json!({ "assets_under_management" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_owned());
        self
    }
    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = Some(secret.to_owned());
        self
    }
    pub fn create_link_customization(mut self, create_link_customization: bool) -> Self {
        self.create_link_customization = Some(create_link_customization);
        self
    }
    pub fn logo(mut self, logo: &str) -> Self {
        self.logo = Some(logo.to_owned());
        self
    }
    pub fn technical_contact(
        mut self,
        technical_contact: PartnerEndCustomerTechnicalContact,
    ) -> Self {
        self.technical_contact = Some(technical_contact);
        self
    }
    pub fn billing_contact(
        mut self,
        billing_contact: PartnerEndCustomerBillingContact,
    ) -> Self {
        self.billing_contact = Some(billing_contact);
        self
    }
    pub fn customer_support_info(
        mut self,
        customer_support_info: PartnerEndCustomerCustomerSupportInfo,
    ) -> Self {
        self.customer_support_info = Some(customer_support_info);
        self
    }
    pub fn assets_under_management(
        mut self,
        assets_under_management: PartnerEndCustomerAssetsUnderManagement,
    ) -> Self {
        self.assets_under_management = Some(assets_under_management);
        self
    }
}
pub struct PartnerCustomerCreateRequired<'a> {
    pub company_name: &'a str,
    pub is_diligence_attested: bool,
    pub products: &'a [&'a str],
    pub legal_entity_name: &'a str,
    pub website: &'a str,
    pub application_name: &'a str,
    pub address: PartnerEndCustomerAddress,
    pub is_bank_addendum_completed: bool,
}
impl<'a> PartnerCustomerCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for PartnerCustomerCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<PartnerCustomerCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}