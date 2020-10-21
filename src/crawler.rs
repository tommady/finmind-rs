use crate::schema::{Result, TaiwanStockPriceArgs, TaiwanStockPriceResponse};

const V3_DEFAULT_URL: &str = "https://api.finmindtrade.com/api/v3/data";
const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";

pub fn taiwan_stock_price_blocking<A>(args: A) -> Result<TaiwanStockPriceResponse>
where
    A: Into<TaiwanStockPriceArgs>,
{
    let args = args.into();
    let url = reqwest::Url::parse_with_params(
        V3_DEFAULT_URL,
        &[
            ("stock_id", args.stock_id),
            ("dataset", "TaiwanStockPrice"),
            (
                "date",
                &args.start_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            (
                "end_date",
                &args.end_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            ("user_id", args.user_id),
            ("password", args.password),
        ],
    )?;

    let resp = reqwest::blocking::get(url)?;
    if resp.status().is_success() {
        Ok(resp.json()?)
    } else {
        Err(resp.text()?.into())
    }
}

pub async fn taiwan_stock_price_async<A>(args: A) -> Result<TaiwanStockPriceResponse>
where
    A: Into<TaiwanStockPriceArgs>,
{
    let args = args.into();
    let url = reqwest::Url::parse_with_params(
        V3_DEFAULT_URL,
        &[
            ("stock_id", args.stock_id),
            ("dataset", "TaiwanStockPrice"),
            (
                "date",
                &args.start_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            (
                "end_date",
                &args.end_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            ("user_id", args.user_id),
            ("password", args.password),
        ],
    )?;

    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(resp.text().await?.into())
    }
}