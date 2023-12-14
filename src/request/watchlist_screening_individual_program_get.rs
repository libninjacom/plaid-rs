use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_program_get`].

On request success, this will return a [`WatchlistScreeningIndividualProgramGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualProgramGetRequest {
    pub watchlist_program_id: String,
}
impl WatchlistScreeningIndividualProgramGetRequest {}
impl FluentRequest<'_, WatchlistScreeningIndividualProgramGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualProgramGetRequest> {
    type Output = httpclient::InMemoryResult<
        WatchlistScreeningIndividualProgramGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/watchlist_screening/individual/program/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}