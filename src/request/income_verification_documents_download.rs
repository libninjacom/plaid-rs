use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct IncomeVerificationDocumentsDownloadRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
    pub document_id: Option<String>,
}
impl<'a> IncomeVerificationDocumentsDownloadRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .post("/income/verification/documents/download");
        if let Some(ref unwrapped) = self.income_verification_id {
            r = r.json(json!({ "income_verification_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.document_id {
            r = r.json(json!({ "document_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn document_id(mut self, document_id: &str) -> Self {
        self.document_id = Some(document_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for IncomeVerificationDocumentsDownloadRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}