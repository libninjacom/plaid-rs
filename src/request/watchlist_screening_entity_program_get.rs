use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningEntityProgramGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub entity_watchlist_program_id: String,
}
impl<'a> WatchlistScreeningEntityProgramGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WatchlistScreeningEntityProgramGetResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/entity/program/get");
        r = r
            .json(
                json!(
                    { "entity_watchlist_program_id" : self.entity_watchlist_program_id }
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningEntityProgramGetRequest<'a> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningEntityProgramGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}