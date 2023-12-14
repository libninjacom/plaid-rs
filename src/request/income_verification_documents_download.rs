use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationDocumentsDownloadRequest {
    pub access_token: Option<String>,
    pub document_id: Option<String>,
    pub income_verification_id: Option<String>,
}
impl IncomeVerificationDocumentsDownloadRequest {}
impl FluentRequest<'_, IncomeVerificationDocumentsDownloadRequest> {
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    pub fn document_id(mut self, document_id: &str) -> Self {
        self.params.document_id = Some(document_id.to_owned());
        self
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.params.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationDocumentsDownloadRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/income/verification/documents/download";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}