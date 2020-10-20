use chrono::{TimeZone, Utc};
use finmind::crawler;

fn main() {
    match crawler::taiwan_stock_price_blocking((
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    )) {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("{}", e),
    }
}
