use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferQuestionnaireCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub originator_client_id: String,
    pub redirect_uri: String,
}
impl<'a> TransferQuestionnaireCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferQuestionnaireCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/transfer/questionnaire/create");
        r = r.json(json!({ "originator_client_id" : self.originator_client_id }));
        r = r.json(json!({ "redirect_uri" : self.redirect_uri }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for TransferQuestionnaireCreateRequest<'a> {
    type Output = crate::Result<TransferQuestionnaireCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
