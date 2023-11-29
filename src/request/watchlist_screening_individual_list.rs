use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub status: Option<String>,
    pub watchlist_program_id: String,
}
impl<'a> WatchlistScreeningIndividualListRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningIndividualListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/list");
        if let Some(ref unwrapped) = self.assignee {
            r = r.json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.json(json!({ "status" : unwrapped }));
        }
        r = r.json(json!({ "watchlist_program_id" : self.watchlist_program_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualListRequest<'a> {
    type Output = crate::Result<WatchlistScreeningIndividualListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
