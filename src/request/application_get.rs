use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ApplicationGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub application_id: String,
}
impl<'a> ApplicationGetRequest<'a> {
    pub async fn send(self) -> crate::Result<ApplicationGetResponse> {
        let mut r = self.http_client.client.post("/application/get");
        r = r.json(json!({ "application_id" : self.application_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for ApplicationGetRequest<'a> {
    type Output = crate::Result<ApplicationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
