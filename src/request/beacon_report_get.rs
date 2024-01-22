use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_report_get`].

On request success, this will return a [`BeaconReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportGetRequest {
    pub beacon_report_id: String,
}
impl BeaconReportGetRequest {}
impl FluentRequest<'_, BeaconReportGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconReportGetRequest> {
    type Output = httpclient::InMemoryResult<BeaconReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "beacon_report_id" : self.params.beacon_report_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}