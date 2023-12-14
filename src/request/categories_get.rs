use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::categories_get`].

On request success, this will return a [`CategoriesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriesGetRequest {}
impl CategoriesGetRequest {}
impl FluentRequest<'_, CategoriesGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CategoriesGetRequest> {
    type Output = httpclient::InMemoryResult<CategoriesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/categories/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}