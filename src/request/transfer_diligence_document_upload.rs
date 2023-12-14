use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_diligence_document_upload`].

On request success, this will return a [`TransferDiligenceDocumentUploadResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDiligenceDocumentUploadRequest {
    pub file: String,
    pub originator_client_id: String,
    pub purpose: String,
}
impl TransferDiligenceDocumentUploadRequest {}
impl FluentRequest<'_, TransferDiligenceDocumentUploadRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferDiligenceDocumentUploadRequest> {
    type Output = httpclient::InMemoryResult<TransferDiligenceDocumentUploadResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/diligence/document/upload";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}