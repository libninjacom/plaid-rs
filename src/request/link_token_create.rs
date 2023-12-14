use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    pub access_token: Option<String>,
    pub access_tokens: Option<Vec<String>>,
    pub account_filters: Option<LinkTokenAccountFilters>,
    pub additional_consented_products: Option<Vec<String>>,
    pub android_package_name: Option<String>,
    pub auth: Option<LinkTokenCreateRequestAuth>,
    pub base_report: Option<LinkTokenCreateRequestBaseReport>,
    pub card_switch: Option<LinkTokenCreateCardSwitch>,
    pub client_name: String,
    pub consumer_report_permissible_purpose: Option<String>,
    pub country_codes: Vec<String>,
    pub cra_enabled: Option<bool>,
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    pub employment: Option<LinkTokenCreateRequestEmployment>,
    pub eu_config: Option<LinkTokenEuConfig>,
    pub hosted_link: Option<LinkTokenCreateHostedLink>,
    pub identity_verification: Option<LinkTokenCreateRequestIdentityVerification>,
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    pub institution_id: Option<String>,
    pub investments: Option<LinkTokenInvestments>,
    pub investments_auth: Option<LinkTokenInvestmentsAuth>,
    pub language: String,
    pub link_customization_name: Option<String>,
    pub optional_products: Option<Vec<String>>,
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    pub products: Option<Vec<String>>,
    pub redirect_uri: Option<String>,
    pub required_if_supported_products: Option<Vec<String>>,
    pub statements: Option<LinkTokenCreateRequestStatements>,
    pub transactions: Option<LinkTokenTransactions>,
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    pub update: Option<LinkTokenCreateRequestUpdate>,
    pub user: LinkTokenCreateRequestUser,
    pub user_token: Option<String>,
    pub webhook: Option<String>,
}
impl LinkTokenCreateRequest {}
pub struct LinkTokenCreateRequired<'a> {
    pub client_name: &'a str,
    pub country_codes: &'a [&'a str],
    pub language: &'a str,
    pub user: LinkTokenCreateRequestUser,
}
impl<'a> LinkTokenCreateRequired<'a> {}
impl FluentRequest<'_, LinkTokenCreateRequest> {
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn account_filters(mut self, account_filters: LinkTokenAccountFilters) -> Self {
        self.params.account_filters = Some(account_filters);
        self
    }
    pub fn additional_consented_products(
        mut self,
        additional_consented_products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .additional_consented_products = Some(
            additional_consented_products
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn android_package_name(mut self, android_package_name: &str) -> Self {
        self.params.android_package_name = Some(android_package_name.to_owned());
        self
    }
    pub fn auth(mut self, auth: LinkTokenCreateRequestAuth) -> Self {
        self.params.auth = Some(auth);
        self
    }
    pub fn base_report(mut self, base_report: LinkTokenCreateRequestBaseReport) -> Self {
        self.params.base_report = Some(base_report);
        self
    }
    pub fn card_switch(mut self, card_switch: LinkTokenCreateCardSwitch) -> Self {
        self.params.card_switch = Some(card_switch);
        self
    }
    pub fn consumer_report_permissible_purpose(
        mut self,
        consumer_report_permissible_purpose: &str,
    ) -> Self {
        self
            .params
            .consumer_report_permissible_purpose = Some(
            consumer_report_permissible_purpose.to_owned(),
        );
        self
    }
    pub fn cra_enabled(mut self, cra_enabled: bool) -> Self {
        self.params.cra_enabled = Some(cra_enabled);
        self
    }
    pub fn deposit_switch(
        mut self,
        deposit_switch: LinkTokenCreateRequestDepositSwitch,
    ) -> Self {
        self.params.deposit_switch = Some(deposit_switch);
        self
    }
    pub fn employment(mut self, employment: LinkTokenCreateRequestEmployment) -> Self {
        self.params.employment = Some(employment);
        self
    }
    pub fn eu_config(mut self, eu_config: LinkTokenEuConfig) -> Self {
        self.params.eu_config = Some(eu_config);
        self
    }
    pub fn hosted_link(mut self, hosted_link: LinkTokenCreateHostedLink) -> Self {
        self.params.hosted_link = Some(hosted_link);
        self
    }
    pub fn identity_verification(
        mut self,
        identity_verification: LinkTokenCreateRequestIdentityVerification,
    ) -> Self {
        self.params.identity_verification = Some(identity_verification);
        self
    }
    pub fn income_verification(
        mut self,
        income_verification: LinkTokenCreateRequestIncomeVerification,
    ) -> Self {
        self.params.income_verification = Some(income_verification);
        self
    }
    pub fn institution_data(
        mut self,
        institution_data: LinkTokenCreateInstitutionData,
    ) -> Self {
        self.params.institution_data = Some(institution_data);
        self
    }
    pub fn institution_id(mut self, institution_id: &str) -> Self {
        self.params.institution_id = Some(institution_id.to_owned());
        self
    }
    pub fn investments(mut self, investments: LinkTokenInvestments) -> Self {
        self.params.investments = Some(investments);
        self
    }
    pub fn investments_auth(
        mut self,
        investments_auth: LinkTokenInvestmentsAuth,
    ) -> Self {
        self.params.investments_auth = Some(investments_auth);
        self
    }
    pub fn link_customization_name(mut self, link_customization_name: &str) -> Self {
        self.params.link_customization_name = Some(link_customization_name.to_owned());
        self
    }
    pub fn optional_products(
        mut self,
        optional_products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .optional_products = Some(
            optional_products.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn payment_initiation(
        mut self,
        payment_initiation: LinkTokenCreateRequestPaymentInitiation,
    ) -> Self {
        self.params.payment_initiation = Some(payment_initiation);
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
    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.params.redirect_uri = Some(redirect_uri.to_owned());
        self
    }
    pub fn required_if_supported_products(
        mut self,
        required_if_supported_products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .required_if_supported_products = Some(
            required_if_supported_products
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn statements(mut self, statements: LinkTokenCreateRequestStatements) -> Self {
        self.params.statements = Some(statements);
        self
    }
    pub fn transactions(mut self, transactions: LinkTokenTransactions) -> Self {
        self.params.transactions = Some(transactions);
        self
    }
    pub fn transfer(mut self, transfer: LinkTokenCreateRequestTransfer) -> Self {
        self.params.transfer = Some(transfer);
        self
    }
    pub fn update(mut self, update: LinkTokenCreateRequestUpdate) -> Self {
        self.params.update = Some(update);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<LinkTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/link/token/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}