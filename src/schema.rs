use chrono::{Date, Utc};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "snake_case")]
pub struct TaiwanStockPriceData {
    pub date: String,
    pub stock_id: String,
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
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "snake_case")]
pub struct TaiwanStockPriceResponse {
    pub msg: String,
    pub status: usize,
    pub data: Vec<TaiwanStockPriceData>,
}

pub struct TaiwanStockPriceArgs {
    pub stock_id: &'static str,
    pub start_date: Date<Utc>,
    pub end_date: Date<Utc>,
    pub user_id: &'static str,
    pub password: &'static str,
}

impl Default for TaiwanStockPriceArgs {
    fn default() -> Self {
        TaiwanStockPriceArgs {
            stock_id: "",
            start_date: chrono::offset::Utc::today(),
            end_date: chrono::offset::Utc::today(),
            user_id: "",
            password: "",
        }
    }
}

impl From<()> for TaiwanStockPriceArgs {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<&'static str> for TaiwanStockPriceArgs {
    fn from(stock_id: &'static str) -> Self {
        Self {
            stock_id: stock_id,
            ..Self::default()
        }
    }
}

impl From<(&'static str, Date<Utc>)> for TaiwanStockPriceArgs {
    fn from((stock_id, start_date): (&'static str, Date<Utc>)) -> Self {
        Self {
            stock_id: stock_id,
            start_date: start_date,
            ..Self::default()
        }
    }
}

impl From<(&'static str, Date<Utc>, Date<Utc>)> for TaiwanStockPriceArgs {
    fn from((stock_id, start_date, end_date): (&'static str, Date<Utc>, Date<Utc>)) -> Self {
        Self {
            stock_id: stock_id,
            start_date: start_date,
            end_date: end_date,
            ..Self::default()
        }
    }
}

impl
    From<(
        &'static str,
        Date<Utc>,
        Date<Utc>,
        &'static str,
        &'static str,
    )> for TaiwanStockPriceArgs
{
    fn from(
        (stock_id, start_date, end_date, user_id, password): (
            &'static str,
            Date<Utc>,
            Date<Utc>,
            &'static str,
            &'static str,
        ),
    ) -> Self {
        Self {
            stock_id: stock_id,
            start_date: start_date,
            end_date: end_date,
            user_id: user_id,
            password: password,
        }
    }
}
