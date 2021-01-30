use chrono::NaiveDate;
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, FinmindError>;

// Trading_Volume(成交量)
// Trading_money(成交金額)
// Trading_turnover(周轉率):週轉率高代表股票交易越活絡
// close(收盤價)
// date(日期)
// max(當日最高價)
// min(當日最低價)
// open(開盤價)
// spread(震幅)
// stock_id(股票代碼)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaiwanStockPrice {
    #[serde(alias = "Trading_Volume")]
    pub trading_volume: u64,
    #[serde(alias = "Trading_money")]
    pub trading_money: u64,
    pub open: f64,
    pub max: f64,
    pub min: f64,
    pub close: f64,
    pub spread: f64,
    #[serde(alias = "Trading_turnover")]
    pub trading_turnover: f64,
    pub date: NaiveDate,
    pub stock_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaiwanStockInstitutionalInvestorsBuySell {
    pub buy: u64,
    pub name: String,
    pub sell: u64,
    pub date: NaiveDate,
    pub stock_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaiwanStockTotalInstitutionalInvestors {
    pub buy: u64,
    pub name: String,
    pub sell: u64,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaiwanStockShareholding {
    pub date: NaiveDate,
    pub stock_id: String,
    pub stock_name: String,
    #[serde(alias = "InternationalCode")]
    pub international_code: String,
    #[serde(alias = "ForeignInvestmentRemainingShares")]
    pub foreign_investment_remaining_shares: u64,
    #[serde(alias = "ForeignInvestmentShares")]
    pub foreign_investment_shares: u64,
    #[serde(alias = "ForeignInvestmentRemainRatio")]
    pub foreign_investment_remain_ratio: f64,
    #[serde(alias = "ForeignInvestmentSharesRatio")]
    pub foreign_investment_shares_ratio: f64,
    #[serde(alias = "ForeignInvestmentUpperLimitRatio")]
    pub foreign_investment_upper_limit_ratio: f64,
    #[serde(alias = "ChineseInvestmentUpperLimitRatio")]
    pub chinese_investment_upper_limit_ratio: f64,
    #[serde(alias = "NumberOfSharesIssued")]
    pub number_of_shares_issued: u64,
    #[serde(alias = "RecentlyDeclareDate")]
    pub recently_declare_date: NaiveDate,
    #[serde(skip_deserializing)]
    pub note: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaiwanStockMarginPurchaseShortSale {
    pub date: NaiveDate,
    pub stock_id: String,
    #[serde(alias = "MarginPurchaseBuy")]
    pub margin_purchase_buy: u64,
    #[serde(alias = "MarginPurchaseCashRepayment")]
    pub margin_purchase_cash_repayment: u64,
    #[serde(alias = "MarginPurchaseLimit")]
    pub margin_purchase_limit: u64,
    #[serde(alias = "MarginPurchaseSell")]
    pub margin_purchase_sell: u64,
    #[serde(alias = "MarginPurchaseTodayBalance")]
    pub margin_purchase_today_balance: u64,
    #[serde(alias = "MarginPurchaseYesterdayBalance")]
    pub margin_purchase_yesterday_balance: u64,
    #[serde(skip_deserializing, alias = "Note")]
    pub note: String,
    #[serde(alias = "OffsetLoanAndShort")]
    pub offset_loan_and_short: u64,
    #[serde(alias = "ShortSaleBuy")]
    pub short_sale_buy: u64,
    #[serde(alias = "ShortSaleCashRepayment")]
    pub short_sale_cash_repayment: u64,
    #[serde(alias = "ShortSaleLimit")]
    pub short_sale_limit: u64,
    #[serde(alias = "ShortSaleSell")]
    pub short_sale_sell: u64,
    #[serde(alias = "ShortSaleTodayBalance")]
    pub short_sale_today_balance: u64,
    #[serde(alias = "ShortSaleYesterdayBalance")]
    pub short_sale_yesterday_balance: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaiwanStockMonthRevenue {
    pub date: NaiveDate,
    pub stock_id: String,
    pub country: String,
    pub revenue: u64,
    pub revenue_month: u32,
    pub revenue_year: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Data {
    TaiwanStockPrice(TaiwanStockPrice),
    TaiwanStockInstitutionalInvestorsBuySell(TaiwanStockInstitutionalInvestorsBuySell),
    TaiwanStockTotalInstitutionalInvestors(TaiwanStockTotalInstitutionalInvestors),
    TaiwanStockShareholding(TaiwanStockShareholding),
    TaiwanStockMarginPurchaseShortSale(TaiwanStockMarginPurchaseShortSale),
    TaiwanStockMonthRevenue(TaiwanStockMonthRevenue),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Response {
    pub msg: String,
    pub status: usize,
    pub data: Vec<Data>,
}

pub enum Dataset {
    Unknown,
    TaiwanStockPrice,
    TaiwanStockTotalInstitutionalInvestors,
    TaiwanStockInstitutionalInvestorsBuySell,
    TaiwanStockShareholding,
    TaiwanStockMarginPurchaseShortSale,
    TaiwanStockMonthRevenue,
}

impl std::fmt::Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Dataset::Unknown => write!(f, "Unknown"),
            Dataset::TaiwanStockPrice => write!(f, "TaiwanStockPrice"),
            Dataset::TaiwanStockTotalInstitutionalInvestors => {
                write!(f, "TaiwanStockTotalInstitutionalInvestors")
            }
            Dataset::TaiwanStockInstitutionalInvestorsBuySell => {
                write!(f, "TaiwanStockInstitutionalInvestorsBuySell")
            }
            Dataset::TaiwanStockShareholding => write!(f, "TaiwanStockShareholding"),
            Dataset::TaiwanStockMarginPurchaseShortSale => {
                write!(f, "TaiwanStockMarginPurchaseShortSale")
            }
            Dataset::TaiwanStockMonthRevenue => write!(f, "TaiwanStockMonthRevenue"),
        }
    }
}

pub struct Args {
    pub dataset: Dataset,
    pub data_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub token: String,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            dataset: Dataset::Unknown,
            data_id: "".to_owned(),
            start_date: chrono::offset::Utc::today().naive_utc(),
            end_date: chrono::offset::Utc::today().naive_utc(),
            token: "".to_owned(),
        }
    }
}

impl From<()> for Args {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<Dataset> for Args {
    fn from(dataset: Dataset) -> Self {
        Self {
            dataset: dataset,
            ..Self::default()
        }
    }
}

impl From<(Dataset, NaiveDate, NaiveDate)> for Args {
    fn from((dataset, start_date, end_date): (Dataset, NaiveDate, NaiveDate)) -> Self {
        Self {
            dataset: dataset,
            start_date: start_date,
            end_date: end_date,
            ..Self::default()
        }
    }
}

impl From<(Dataset, String, NaiveDate, NaiveDate)> for Args {
    fn from(
        (dataset, data_id, start_date, end_date): (Dataset, String, NaiveDate, NaiveDate),
    ) -> Self {
        Self {
            dataset: dataset,
            data_id: data_id,
            start_date: start_date,
            end_date: end_date,
            ..Self::default()
        }
    }
}

impl From<(Dataset, NaiveDate, NaiveDate, String)> for Args {
    fn from(
        (dataset, start_date, end_date, token): (Dataset, NaiveDate, NaiveDate, String),
    ) -> Self {
        Self {
            dataset: dataset,
            start_date: start_date,
            end_date: end_date,
            token: token,
            ..Self::default()
        }
    }
}

impl From<(Dataset, String, NaiveDate, NaiveDate, String)> for Args {
    fn from(
        (dataset, stock_id, start_date, end_date, token): (
            Dataset,
            String,
            NaiveDate,
            NaiveDate,
            String,
        ),
    ) -> Self {
        Self {
            dataset: dataset,
            data_id: stock_id,
            start_date: start_date,
            end_date: end_date,
            token: token,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorResponse {
    pub status: usize,
    pub msg: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "FinmindAPI: {{ status:{}, msg:{} }}",
            self.status, self.msg,
        )
    }
}

impl std::error::Error for ErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum FinmindError {
    // error from serde_json lib
    SerdeJson(serde_json::Error),
    // error from reqwest lib
    Reqwest(reqwest::Error),
    // Url parsing error
    Url(url::ParseError),
    // errors from http response status
    // 402 response status
    RateLimitReached,
    // 400 response status
    BadRequest,
    // unknown error
    Unknown(ErrorResponse),
}

impl std::fmt::Display for FinmindError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FinmindError::Url(ref e) => write!(f, "Url Parse error: {}", e),
            FinmindError::SerdeJson(ref e) => write!(f, "Serde_json Lib error: {}", e),
            FinmindError::Reqwest(ref e) => write!(f, "Reqwest Lib error: {}", e),
            FinmindError::RateLimitReached => write!(f, "Rate limit reached"),
            FinmindError::BadRequest => write!(f, "Bad Request"),
            FinmindError::Unknown(ref e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for FinmindError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FinmindError::Url(ref e) => Some(e),
            FinmindError::SerdeJson(ref e) => Some(e),
            FinmindError::Reqwest(ref e) => Some(e),
            FinmindError::RateLimitReached => None,
            FinmindError::BadRequest => None,
            FinmindError::Unknown(ref _e) => None,
        }
    }
}

impl From<url::ParseError> for FinmindError {
    fn from(err: url::ParseError) -> FinmindError {
        FinmindError::Url(err)
    }
}

impl From<serde_json::Error> for FinmindError {
    fn from(err: serde_json::Error) -> FinmindError {
        FinmindError::SerdeJson(err)
    }
}

impl From<reqwest::Error> for FinmindError {
    fn from(err: reqwest::Error) -> FinmindError {
        FinmindError::Reqwest(err)
    }
}

impl From<String> for FinmindError {
    fn from(err: String) -> FinmindError {
        FinmindError::Unknown(ErrorResponse {
            status: 500,
            msg: err,
        })
    }
}

impl From<ErrorResponse> for FinmindError {
    fn from(err: ErrorResponse) -> FinmindError {
        match err.status {
            400 => FinmindError::BadRequest,
            402 => FinmindError::RateLimitReached,
            _ => FinmindError::Unknown(err),
        }
    }
}
