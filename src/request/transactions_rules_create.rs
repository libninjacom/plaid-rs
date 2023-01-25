use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsRulesCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub personal_finance_category: String,
    pub rule_details: TransactionsRuleDetails,
}
impl<'a> TransactionsRulesCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TransactionsRulesCreateResponse> {
        let mut r = self.http_client.client.post("/beta/transactions/rules/v1/create");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r
            .json(
                json!({ "personal_finance_category" : self.personal_finance_category }),
            );
        r = r.json(json!({ "rule_details" : self.rule_details }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsRulesCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<TransactionsRulesCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}