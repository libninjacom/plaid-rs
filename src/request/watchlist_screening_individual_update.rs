use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_update`].

On request success, this will return a [`WatchlistScreeningIndividualUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualUpdateRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub reset_fields: Option<Vec<String>>,
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    pub status: Option<String>,
    pub watchlist_screening_id: String,
}
impl WatchlistScreeningIndividualUpdateRequest {}
impl FluentRequest<'_, WatchlistScreeningIndividualUpdateRequest> {
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.params.assignee = Some(assignee.to_owned());
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn reset_fields(
        mut self,
        reset_fields: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .reset_fields = Some(
            reset_fields.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn search_terms(
        mut self,
        search_terms: UpdateIndividualScreeningRequestSearchTerms,
    ) -> Self {
        self.params.search_terms = Some(search_terms);
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.params.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualUpdateRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningIndividualUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/watchlist_screening/individual/update";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}