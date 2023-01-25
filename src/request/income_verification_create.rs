use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IncomeVerificationCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub webhook: String,
    pub precheck_id: Option<String>,
    pub options: Option<IncomeVerificationCreateRequestOptions>,
}
impl<'a> IncomeVerificationCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<IncomeVerificationCreateResponse> {
        let mut r = self.http_client.client.post("/income/verification/create");
        r = r.json(json!({ "webhook" : self.webhook }));
        if let Some(ref unwrapped) = self.precheck_id {
            r = r.json(json!({ "precheck_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn precheck_id(mut self, precheck_id: &str) -> Self {
        self.precheck_id = Some(precheck_id.to_owned());
        self
    }
    pub fn options(mut self, options: IncomeVerificationCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for IncomeVerificationCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<IncomeVerificationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}