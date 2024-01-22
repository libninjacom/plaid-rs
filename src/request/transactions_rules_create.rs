use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_rules_create`].

On request success, this will return a [`TransactionsRulesCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesCreateRequest {
    pub access_token: String,
    pub personal_finance_category: String,
    pub rule_details: TransactionsRuleDetails,
}
impl TransactionsRulesCreateRequest {}
impl FluentRequest<'_, TransactionsRulesCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsRulesCreateRequest> {
    type Output = httpclient::InMemoryResult<TransactionsRulesCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/rules/v1/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r
                .json(
                    json!(
                        { "personal_finance_category" : self.params
                        .personal_finance_category }
                    ),
                );
            r = r.json(json!({ "rule_details" : self.params.rule_details }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}