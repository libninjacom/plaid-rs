use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_program_list`].

On request success, this will return a [`WatchlistScreeningIndividualProgramListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualProgramListRequest {
    pub cursor: Option<String>,
}
impl WatchlistScreeningIndividualProgramListRequest {}
impl FluentRequest<'_, WatchlistScreeningIndividualProgramListRequest> {
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualProgramListRequest> {
    type Output = httpclient::InMemoryResult<
        WatchlistScreeningIndividualProgramListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/program/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}