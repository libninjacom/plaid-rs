use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_program_get`].

On request success, this will return a [`WatchlistScreeningEntityProgramGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityProgramGetRequest {
    pub entity_watchlist_program_id: String,
}
impl WatchlistScreeningEntityProgramGetRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityProgramGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityProgramGetRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningEntityProgramGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/program/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "entity_watchlist_program_id" : self.params
                        .entity_watchlist_program_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}