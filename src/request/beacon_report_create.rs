use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::beacon_report_create`].

On request success, this will return a [`BeaconReportCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportCreateRequest {
    pub beacon_user_id: String,
    pub fraud_amount: Option<FraudAmount>,
    pub fraud_date: chrono::NaiveDate,
    pub type_: String,
}
impl BeaconReportCreateRequest {}
impl FluentRequest<'_, BeaconReportCreateRequest> {
    pub fn fraud_amount(mut self, fraud_amount: FraudAmount) -> Self {
        self.params.fraud_amount = Some(fraud_amount);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconReportCreateRequest> {
    type Output = httpclient::InMemoryResult<BeaconReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/beacon/report/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}