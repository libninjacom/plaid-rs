use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::institutions_get_by_id`].

On request success, this will return a [`InstitutionsGetByIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequest {
    pub country_codes: Vec<String>,
    pub institution_id: String,
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
impl InstitutionsGetByIdRequest {}
impl FluentRequest<'_, InstitutionsGetByIdRequest> {
    pub fn options(mut self, options: InstitutionsGetByIdRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InstitutionsGetByIdRequest> {
    type Output = httpclient::InMemoryResult<InstitutionsGetByIdResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/institutions/get_by_id";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}