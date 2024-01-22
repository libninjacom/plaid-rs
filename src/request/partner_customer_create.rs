use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::partner_customer_create`].

On request success, this will return a [`PartnerCustomerCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCustomerCreateRequest {
    pub address: PartnerEndCustomerAddress,
    pub application_name: String,
    pub assets_under_management: Option<PartnerEndCustomerAssetsUnderManagement>,
    pub billing_contact: Option<PartnerEndCustomerBillingContact>,
    pub client_id: Option<String>,
    pub company_name: String,
    pub create_link_customization: Option<bool>,
    pub customer_support_info: Option<PartnerEndCustomerCustomerSupportInfo>,
    pub is_bank_addendum_completed: bool,
    pub is_diligence_attested: bool,
    pub legal_entity_name: String,
    pub logo: Option<String>,
    pub products: Option<Vec<String>>,
    pub redirect_uris: Option<Vec<String>>,
    pub registration_number: Option<String>,
    pub secret: Option<String>,
    pub technical_contact: Option<PartnerEndCustomerTechnicalContact>,
    pub website: String,
}
impl PartnerCustomerCreateRequest {}
pub struct PartnerCustomerCreateRequired<'a> {
    pub address: PartnerEndCustomerAddress,
    pub application_name: &'a str,
    pub company_name: &'a str,
    pub is_bank_addendum_completed: bool,
    pub is_diligence_attested: bool,
    pub legal_entity_name: &'a str,
    pub website: &'a str,
}
impl<'a> PartnerCustomerCreateRequired<'a> {}
impl FluentRequest<'_, PartnerCustomerCreateRequest> {
    pub fn assets_under_management(
        mut self,
        assets_under_management: PartnerEndCustomerAssetsUnderManagement,
    ) -> Self {
        self.params.assets_under_management = Some(assets_under_management);
        self
    }
    pub fn billing_contact(
        mut self,
        billing_contact: PartnerEndCustomerBillingContact,
    ) -> Self {
        self.params.billing_contact = Some(billing_contact);
        self
    }
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.params.client_id = Some(client_id.to_owned());
        self
    }
    pub fn create_link_customization(mut self, create_link_customization: bool) -> Self {
        self.params.create_link_customization = Some(create_link_customization);
        self
    }
    pub fn customer_support_info(
        mut self,
        customer_support_info: PartnerEndCustomerCustomerSupportInfo,
    ) -> Self {
        self.params.customer_support_info = Some(customer_support_info);
        self
    }
    pub fn logo(mut self, logo: &str) -> Self {
        self.params.logo = Some(logo.to_owned());
        self
    }
    pub fn products(
        mut self,
        products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .products = Some(
            products.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn redirect_uris(
        mut self,
        redirect_uris: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .redirect_uris = Some(
            redirect_uris.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn registration_number(mut self, registration_number: &str) -> Self {
        self.params.registration_number = Some(registration_number.to_owned());
        self
    }
    pub fn secret(mut self, secret: &str) -> Self {
        self.params.secret = Some(secret.to_owned());
        self
    }
    pub fn technical_contact(
        mut self,
        technical_contact: PartnerEndCustomerTechnicalContact,
    ) -> Self {
        self.params.technical_contact = Some(technical_contact);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PartnerCustomerCreateRequest> {
    type Output = httpclient::InMemoryResult<PartnerCustomerCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/partner/customer/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "address" : self.params.address }));
            r = r.json(json!({ "application_name" : self.params.application_name }));
            if let Some(ref unwrapped) = self.params.assets_under_management {
                r = r.json(json!({ "assets_under_management" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.billing_contact {
                r = r.json(json!({ "billing_contact" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_id {
                r = r.json(json!({ "client_id" : unwrapped }));
            }
            r = r.json(json!({ "company_name" : self.params.company_name }));
            if let Some(ref unwrapped) = self.params.create_link_customization {
                r = r.json(json!({ "create_link_customization" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.customer_support_info {
                r = r.json(json!({ "customer_support_info" : unwrapped }));
            }
            r = r
                .json(
                    json!(
                        { "is_bank_addendum_completed" : self.params
                        .is_bank_addendum_completed }
                    ),
                );
            r = r
                .json(
                    json!(
                        { "is_diligence_attested" : self.params.is_diligence_attested }
                    ),
                );
            r = r.json(json!({ "legal_entity_name" : self.params.legal_entity_name }));
            if let Some(ref unwrapped) = self.params.logo {
                r = r.json(json!({ "logo" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.products {
                r = r.json(json!({ "products" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.redirect_uris {
                r = r.json(json!({ "redirect_uris" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.registration_number {
                r = r.json(json!({ "registration_number" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.secret {
                r = r.json(json!({ "secret" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.technical_contact {
                r = r.json(json!({ "technical_contact" : unwrapped }));
            }
            r = r.json(json!({ "website" : self.params.website }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}