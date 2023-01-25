
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IndividualWatchlistCode {
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_CIA")]
    IzCia,
    #[serde(rename = "IZ_IPL")]
    IzIpl,
    #[serde(rename = "IZ_PEP")]
    IzPep,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "IZ_WBK")]
    IzWbk,
    #[serde(rename = "UK_HMC")]
    UkHmc,
    #[serde(rename = "US_DPL")]
    UsDpl,
    #[serde(rename = "US_DTC")]
    UsDtc,
    #[serde(rename = "US_FBI")]
    UsFbi,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_ISN")]
    UsIsn,
    #[serde(rename = "US_MBS")]
    UsMbs,
    #[serde(rename = "US_PLC")]
    UsPlc,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "SG_SOF")]
    SgSof,
    #[serde(rename = "TR_TWL")]
    TrTwl,
    #[serde(rename = "TR_DFD")]
    TrDfd,
    #[serde(rename = "TR_FOR")]
    TrFor,
    #[serde(rename = "TR_WMD")]
    TrWmd,
    #[serde(rename = "TR_CMB")]
    TrCmb,
}