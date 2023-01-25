
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum IdNumberType {
    #[serde(rename = "ar_dni")]
    ArDni,
    #[serde(rename = "au_drivers_license")]
    AuDriversLicense,
    #[serde(rename = "au_passport")]
    AuPassport,
    #[serde(rename = "br_cpf")]
    BrCpf,
    #[serde(rename = "ca_sin")]
    CaSin,
    #[serde(rename = "cl_run")]
    ClRun,
    #[serde(rename = "cn_resident_card")]
    CnResidentCard,
    #[serde(rename = "co_nit")]
    CoNit,
    #[serde(rename = "dk_cpr")]
    DkCpr,
    #[serde(rename = "eg_national_id")]
    EgNationalId,
    #[serde(rename = "es_dni")]
    EsDni,
    #[serde(rename = "es_nie")]
    EsNie,
    #[serde(rename = "hk_hkid")]
    HkHkid,
    #[serde(rename = "in_pan")]
    InPan,
    #[serde(rename = "it_cf")]
    ItCf,
    #[serde(rename = "jo_civil_id")]
    JoCivilId,
    #[serde(rename = "jp_my_number")]
    JpMyNumber,
    #[serde(rename = "ke_huduma_namba")]
    KeHudumaNamba,
    #[serde(rename = "kw_civil_id")]
    KwCivilId,
    #[serde(rename = "mx_curp")]
    MxCurp,
    #[serde(rename = "mx_rfc")]
    MxRfc,
    #[serde(rename = "my_nric")]
    MyNric,
    #[serde(rename = "ng_nin")]
    NgNin,
    #[serde(rename = "nz_drivers_license")]
    NzDriversLicense,
    #[serde(rename = "om_civil_id")]
    OmCivilId,
    #[serde(rename = "ph_psn")]
    PhPsn,
    #[serde(rename = "pl_pesel")]
    PlPesel,
    #[serde(rename = "ro_cnp")]
    RoCnp,
    #[serde(rename = "sa_national_id")]
    SaNationalId,
    #[serde(rename = "se_pin")]
    SePin,
    #[serde(rename = "sg_nric")]
    SgNric,
    #[serde(rename = "tr_tc_kimlik")]
    TrTcKimlik,
    #[serde(rename = "us_ssn")]
    UsSsn,
    #[serde(rename = "us_ssn_last_4")]
    UsSsnLast4,
    #[serde(rename = "za_smart_id")]
    ZaSmartId,
}