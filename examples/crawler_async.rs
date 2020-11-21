use chrono::{TimeZone, Utc};
use finmind::crawler;
use finmind::schema::Dataset;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match crawler::request_async((
        Dataset::TaiwanStockPrice,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    ))
    .await
    {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("{}", e),
    }

    Ok(())
}
