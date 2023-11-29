use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct InstitutionsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub count: i64,
    pub country_codes: Vec<String>,
    pub offset: i64,
    pub options: Option<InstitutionsGetRequestOptions>,
}
impl<'a> InstitutionsGetRequest<'a> {
    pub async fn send(self) -> crate::Result<InstitutionsGetResponse> {
        let mut r = self.http_client.client.post("/institutions/get");
        r = r.json(json!({ "count" : self.count }));
        r = r.json(json!({ "country_codes" : self.country_codes }));
        r = r.json(json!({ "offset" : self.offset }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: InstitutionsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for InstitutionsGetRequest<'a> {
    type Output = crate::Result<InstitutionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
