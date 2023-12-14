use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/beta/transactions/rules/v1/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}