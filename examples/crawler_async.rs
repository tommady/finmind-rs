use chrono::NaiveDate;
use finmind::crawler;
use finmind::schema::Dataset;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match crawler::request_async((
        Dataset::TaiwanStockPrice,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ))
    .await
    {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("{}", e),
    }

    Ok(())
}
