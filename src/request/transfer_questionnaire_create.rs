use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_questionnaire_create`].

On request success, this will return a [`TransferQuestionnaireCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferQuestionnaireCreateRequest {
    pub originator_client_id: String,
    pub redirect_uri: String,
}
impl TransferQuestionnaireCreateRequest {}
impl FluentRequest<'_, TransferQuestionnaireCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferQuestionnaireCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferQuestionnaireCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/questionnaire/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!({ "originator_client_id" : self.params.originator_client_id }),
                );
            r = r.json(json!({ "redirect_uri" : self.params.redirect_uri }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}