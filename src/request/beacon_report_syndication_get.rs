use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_report_syndication_get`].

On request success, this will return a [`BeaconReportSyndicationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportSyndicationGetRequest {
    pub beacon_report_syndication_id: String,
}
impl BeaconReportSyndicationGetRequest {}
impl FluentRequest<'_, BeaconReportSyndicationGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, BeaconReportSyndicationGetRequest> {
    type Output = httpclient::InMemoryResult<BeaconReportSyndicationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report_syndication/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "beacon_report_syndication_id" : self.params
                        .beacon_report_syndication_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}