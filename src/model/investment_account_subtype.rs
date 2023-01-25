
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentAccountSubtype {
    #[serde(rename = "529")]
    InvestmentAccountSubtype529,
    #[serde(rename = "401a")]
    InvestmentAccountSubtype401A,
    #[serde(rename = "401k")]
    InvestmentAccountSubtype401K,
    #[serde(rename = "403B")]
    InvestmentAccountSubtype403B,
    #[serde(rename = "457b")]
    InvestmentAccountSubtype457B,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "crypto exchange")]
    CryptoExchange,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "non-custodial wallet")]
    NonCustodialWallet,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "pension")]
    Pension,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
    #[serde(rename = "qshr")]
    Qshr,
    #[serde(rename = "rdsp")]
    Rdsp,
    #[serde(rename = "resp")]
    Resp,
    #[serde(rename = "retirement")]
    Retirement,
    #[serde(rename = "rlif")]
    Rlif,
    #[serde(rename = "roth")]
    Roth,
    #[serde(rename = "roth 401k")]
    Roth401K,
    #[serde(rename = "rrif")]
    Rrif,
    #[serde(rename = "rrsp")]
    Rrsp,
    #[serde(rename = "sarsep")]
    Sarsep,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "tfsa")]
    Tfsa,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "ugma")]
    Ugma,
    #[serde(rename = "utma")]
    Utma,
    #[serde(rename = "variable annuity")]
    VariableAnnuity,
    #[serde(rename = "all")]
    All,
}