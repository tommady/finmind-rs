use crate::schema::{Args, ErrorResponse, Response, Result};

// const V3_DEFAULT_URL: &str = "https://api.finmindtrade.com/api/v3/data";
const V4_DEFAULT_URL: &str = "https://api.finmindtrade.com/api/v4/data";
const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";

pub fn request_blocking<A>(args: A) -> Result<Response>
where
    A: Into<Args>,
{
    let args = args.into();
    let url = reqwest::Url::parse_with_params(
        V4_DEFAULT_URL,
        &[
            ("data_id", args.data_id),
            ("dataset", args.dataset.to_string()),
            (
                "start_date",
                args.start_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            (
                "end_date",
                args.end_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            ("token", args.token),
        ],
    )?;

    let resp = reqwest::blocking::get(url)?;
    if resp.status().is_success() {
        let res: Response = resp.json()?;
        if res.status != 200 {
            return Err(ErrorResponse {
                status: res.status,
                msg: res.msg,
            }
            .into());
        }
        Ok(res)
    } else {
        Err(resp.text()?.into())
    }
}

pub async fn request_async<A>(args: A) -> Result<Response>
where
    A: Into<Args>,
{
    let args = args.into();
    let url = reqwest::Url::parse_with_params(
        V4_DEFAULT_URL,
        &[
            ("data_id", args.data_id),
            ("dataset", args.dataset.to_string()),
            (
                "start_date",
                args.start_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            (
                "end_date",
                args.end_date.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
            ("token", args.token),
        ],
    )?;

    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let res: Response = resp.json().await?;
        if res.status != 200 {
            return Err(ErrorResponse {
                status: res.status,
                msg: res.msg,
            }
            .into());
        }
        Ok(res)
    } else {
        Err(resp.text().await?.into())
    }
}
