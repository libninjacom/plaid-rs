use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_get`].

On request success, this will return a [`WatchlistScreeningIndividualGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualGetRequest {
    pub watchlist_screening_id: String,
}
impl WatchlistScreeningIndividualGetRequest {}
impl FluentRequest<'_, WatchlistScreeningIndividualGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualGetRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningIndividualGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "watchlist_screening_id" : self.params.watchlist_screening_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}