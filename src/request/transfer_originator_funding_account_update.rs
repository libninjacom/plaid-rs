use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_originator_funding_account_update`].

On request success, this will return a [`TransferOriginatorFundingAccountUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorFundingAccountUpdateRequest {
    pub funding_account: TransferFundingAccount,
    pub originator_client_id: String,
}
impl TransferOriginatorFundingAccountUpdateRequest {}
impl FluentRequest<'_, TransferOriginatorFundingAccountUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferOriginatorFundingAccountUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        TransferOriginatorFundingAccountUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/originator/funding_account/update";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "funding_account" : self.params.funding_account }));
            r = r
                .json(
                    json!({ "originator_client_id" : self.params.originator_client_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}