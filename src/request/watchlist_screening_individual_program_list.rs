use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualProgramListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualProgramListRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningIndividualProgramListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/program/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualProgramListRequest<'a> {
    type Output = crate::Result<WatchlistScreeningIndividualProgramListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
