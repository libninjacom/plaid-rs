use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct InstitutionsGetByIdRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub country_codes: Vec<String>,
    pub institution_id: String,
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
impl<'a> InstitutionsGetByIdRequest<'a> {
    pub async fn send(self) -> crate::Result<InstitutionsGetByIdResponse> {
        let mut r = self.http_client.client.post("/institutions/get_by_id");
        r = r.json(json!({ "country_codes" : self.country_codes }));
        r = r.json(json!({ "institution_id" : self.institution_id }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: InstitutionsGetByIdRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for InstitutionsGetByIdRequest<'a> {
    type Output = crate::Result<InstitutionsGetByIdResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
