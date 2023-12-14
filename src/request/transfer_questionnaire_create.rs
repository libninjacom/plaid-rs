use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
        Box::pin(async {
            let url = "/transfer/questionnaire/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}