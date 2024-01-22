use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::income_verification_documents_download`].

On request success, this will return a [`()`].*/
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
        Box::pin(async move {
            let url = "/income/verification/documents/download";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(json!({ "access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.document_id {
                r = r.json(json!({ "document_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.income_verification_id {
                r = r.json(json!({ "income_verification_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}