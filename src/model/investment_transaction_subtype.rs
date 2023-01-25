
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentTransactionSubtype {
    #[serde(rename = "account fee")]
    AccountFee,
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "assignment")]
    Assignment,
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "buy to cover")]
    BuyToCover,
    #[serde(rename = "contribution")]
    Contribution,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "distribution")]
    Distribution,
    #[serde(rename = "dividend")]
    Dividend,
    #[serde(rename = "dividend reinvestment")]
    DividendReinvestment,
    #[serde(rename = "exercise")]
    Exercise,
    #[serde(rename = "expire")]
    Expire,
    #[serde(rename = "fund fee")]
    FundFee,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "interest receivable")]
    InterestReceivable,
    #[serde(rename = "interest reinvestment")]
    InterestReinvestment,
    #[serde(rename = "legal fee")]
    LegalFee,
    #[serde(rename = "loan payment")]
    LoanPayment,
    #[serde(rename = "long-term capital gain")]
    LongTermCapitalGain,
    #[serde(rename = "long-term capital gain reinvestment")]
    LongTermCapitalGainReinvestment,
    #[serde(rename = "management fee")]
    ManagementFee,
    #[serde(rename = "margin expense")]
    MarginExpense,
    #[serde(rename = "merger")]
    Merger,
    #[serde(rename = "miscellaneous fee")]
    MiscellaneousFee,
    #[serde(rename = "non-qualified dividend")]
    NonQualifiedDividend,
    #[serde(rename = "non-resident tax")]
    NonResidentTax,
    #[serde(rename = "pending credit")]
    PendingCredit,
    #[serde(rename = "pending debit")]
    PendingDebit,
    #[serde(rename = "qualified dividend")]
    QualifiedDividend,
    #[serde(rename = "rebalance")]
    Rebalance,
    #[serde(rename = "return of principal")]
    ReturnOfPrincipal,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell short")]
    SellShort,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "short-term capital gain")]
    ShortTermCapitalGain,
    #[serde(rename = "short-term capital gain reinvestment")]
    ShortTermCapitalGainReinvestment,
    #[serde(rename = "spin off")]
    SpinOff,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "stock distribution")]
    StockDistribution,
    #[serde(rename = "tax")]
    Tax,
    #[serde(rename = "tax withheld")]
    TaxWithheld,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer fee")]
    TransferFee,
    #[serde(rename = "trust fee")]
    TrustFee,
    #[serde(rename = "unqualified gain")]
    UnqualifiedGain,
    #[serde(rename = "withdrawal")]
    Withdrawal,
}