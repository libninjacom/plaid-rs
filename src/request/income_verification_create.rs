use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IncomeVerificationCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub options: Option<IncomeVerificationCreateRequestOptions>,
    pub precheck_id: Option<String>,
    pub webhook: String,
}
impl<'a> IncomeVerificationCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<IncomeVerificationCreateResponse> {
        let mut r = self.http_client.client.post("/income/verification/create");
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.precheck_id {
            r = r.json(json!({ "precheck_id" : unwrapped }));
        }
        r = r.json(json!({ "webhook" : self.webhook }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: IncomeVerificationCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn precheck_id(mut self, precheck_id: &str) -> Self {
        self.precheck_id = Some(precheck_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for IncomeVerificationCreateRequest<'a> {
    type Output = crate::Result<IncomeVerificationCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
