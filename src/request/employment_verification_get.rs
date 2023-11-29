use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct EmploymentVerificationGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> EmploymentVerificationGetRequest<'a> {
    pub async fn send(self) -> crate::Result<EmploymentVerificationGetResponse> {
        let mut r = self.http_client.client.post("/employment/verification/get");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for EmploymentVerificationGetRequest<'a> {
    type Output = crate::Result<EmploymentVerificationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
