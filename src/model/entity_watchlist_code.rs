
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityWatchlistCode {
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_SOE")]
    IzSoe,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "IZ_WBK")]
    IzWbk,
    #[serde(rename = "US_CAP")]
    UsCap,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_MBS")]
    UsMbs,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "US_CMC")]
    UsCmc,
    #[serde(rename = "US_UVL")]
    UsUvl,
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "UK_HMC")]
    UkHmc,
}