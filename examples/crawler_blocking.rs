use chrono::{TimeZone, Utc};
use finmind::crawler;
use finmind::schema::Dataset;

fn main() {
    match crawler::request_blocking((
        Dataset::TaiwanStockPrice,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    )) {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("{}", e),
    }
}
