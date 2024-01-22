use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_diligence_submit`].

On request success, this will return a [`TransferDiligenceSubmitResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDiligenceSubmitRequest {
    pub originator_client_id: String,
    pub originator_diligence: TransferOriginatorDiligence,
}
impl TransferDiligenceSubmitRequest {}
impl FluentRequest<'_, TransferDiligenceSubmitRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferDiligenceSubmitRequest> {
    type Output = httpclient::InMemoryResult<TransferDiligenceSubmitResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/diligence/submit";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!({ "originator_client_id" : self.params.originator_client_id }),
                );
            r = r
                .json(
                    json!({ "originator_diligence" : self.params.originator_diligence }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}