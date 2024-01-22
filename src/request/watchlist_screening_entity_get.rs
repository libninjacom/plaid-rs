use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_get`].

On request success, this will return a [`WatchlistScreeningEntityGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityGetRequest {
    pub entity_watchlist_screening_id: String,
}
impl WatchlistScreeningEntityGetRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityGetRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningEntityGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "entity_watchlist_screening_id" : self.params
                        .entity_watchlist_screening_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}