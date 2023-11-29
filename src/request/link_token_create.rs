use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct LinkTokenCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: Option<String>,
    pub account_filters: Option<LinkTokenAccountFilters>,
    pub additional_consented_products: Option<Vec<String>>,
    pub android_package_name: Option<String>,
    pub auth: Option<LinkTokenCreateRequestAuth>,
    pub client_name: String,
    pub country_codes: Vec<String>,
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    pub employment: Option<LinkTokenCreateRequestEmployment>,
    pub eu_config: Option<LinkTokenEuConfig>,
    pub identity_verification: Option<LinkTokenCreateRequestIdentityVerification>,
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    pub institution_id: Option<String>,
    pub investments: Option<LinkTokenInvestments>,
    pub language: String,
    pub link_customization_name: Option<String>,
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    pub products: Option<Vec<String>>,
    pub redirect_uri: Option<String>,
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    pub update: Option<LinkTokenCreateRequestUpdate>,
    pub user: LinkTokenCreateRequestUser,
    pub user_token: Option<String>,
    pub webhook: Option<String>,
}
impl<'a> LinkTokenCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<LinkTokenCreateResponse> {
        let mut r = self.http_client.client.post("/link/token/create");
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_filters {
            r = r.json(json!({ "account_filters" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.additional_consented_products {
            r = r.json(json!({ "additional_consented_products" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.android_package_name {
            r = r.json(json!({ "android_package_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.auth {
            r = r.json(json!({ "auth" : unwrapped }));
        }
        r = r.json(json!({ "client_name" : self.client_name }));
        r = r.json(json!({ "country_codes" : self.country_codes }));
        if let Some(ref unwrapped) = self.deposit_switch {
            r = r.json(json!({ "deposit_switch" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.employment {
            r = r.json(json!({ "employment" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.eu_config {
            r = r.json(json!({ "eu_config" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.identity_verification {
            r = r.json(json!({ "identity_verification" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.income_verification {
            r = r.json(json!({ "income_verification" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.institution_data {
            r = r.json(json!({ "institution_data" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.institution_id {
            r = r.json(json!({ "institution_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.investments {
            r = r.json(json!({ "investments" : unwrapped }));
        }
        r = r.json(json!({ "language" : self.language }));
        if let Some(ref unwrapped) = self.link_customization_name {
            r = r.json(json!({ "link_customization_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_initiation {
            r = r.json(json!({ "payment_initiation" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.products {
            r = r.json(json!({ "products" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.redirect_uri {
            r = r.json(json!({ "redirect_uri" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transfer {
            r = r.json(json!({ "transfer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.update {
            r = r.json(json!({ "update" : unwrapped }));
        }
        r = r.json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.user_token {
            r = r.json(json!({ "user_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.webhook {
            r = r.json(json!({ "webhook" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_filters(mut self, account_filters: LinkTokenAccountFilters) -> Self {
        self.account_filters = Some(account_filters);
        self
    }
    pub fn additional_consented_products(
        mut self,
        additional_consented_products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.additional_consented_products = Some(
            additional_consented_products
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn android_package_name(mut self, android_package_name: &str) -> Self {
        self.android_package_name = Some(android_package_name.to_owned());
        self
    }
    pub fn auth(mut self, auth: LinkTokenCreateRequestAuth) -> Self {
        self.auth = Some(auth);
        self
    }
    pub fn deposit_switch(mut self, deposit_switch: LinkTokenCreateRequestDepositSwitch) -> Self {
        self.deposit_switch = Some(deposit_switch);
        self
    }
    pub fn employment(mut self, employment: LinkTokenCreateRequestEmployment) -> Self {
        self.employment = Some(employment);
        self
    }
    pub fn eu_config(mut self, eu_config: LinkTokenEuConfig) -> Self {
        self.eu_config = Some(eu_config);
        self
    }
    pub fn identity_verification(
        mut self,
        identity_verification: LinkTokenCreateRequestIdentityVerification,
    ) -> Self {
        self.identity_verification = Some(identity_verification);
        self
    }
    pub fn income_verification(
        mut self,
        income_verification: LinkTokenCreateRequestIncomeVerification,
    ) -> Self {
        self.income_verification = Some(income_verification);
        self
    }
    pub fn institution_data(mut self, institution_data: LinkTokenCreateInstitutionData) -> Self {
        self.institution_data = Some(institution_data);
        self
    }
    pub fn institution_id(mut self, institution_id: &str) -> Self {
        self.institution_id = Some(institution_id.to_owned());
        self
    }
    pub fn investments(mut self, investments: LinkTokenInvestments) -> Self {
        self.investments = Some(investments);
        self
    }
    pub fn link_customization_name(mut self, link_customization_name: &str) -> Self {
        self.link_customization_name = Some(link_customization_name.to_owned());
        self
    }
    pub fn payment_initiation(
        mut self,
        payment_initiation: LinkTokenCreateRequestPaymentInitiation,
    ) -> Self {
        self.payment_initiation = Some(payment_initiation);
        self
    }
    pub fn products(mut self, products: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.products = Some(
            products
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.redirect_uri = Some(redirect_uri.to_owned());
        self
    }
    pub fn transfer(mut self, transfer: LinkTokenCreateRequestTransfer) -> Self {
        self.transfer = Some(transfer);
        self
    }
    pub fn update(mut self, update: LinkTokenCreateRequestUpdate) -> Self {
        self.update = Some(update);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
pub struct LinkTokenCreateRequired<'a> {
    pub client_name: &'a str,
    pub country_codes: &'a [&'a str],
    pub language: &'a str,
    pub user: LinkTokenCreateRequestUser,
}
impl<'a> LinkTokenCreateRequired<'a> {}
impl<'a> ::std::future::IntoFuture for LinkTokenCreateRequest<'a> {
    type Output = crate::Result<LinkTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
