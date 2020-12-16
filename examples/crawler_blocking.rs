use chrono::NaiveDate;
use finmind::crawler;
use finmind::schema::Dataset;

fn main() {
    match crawler::request_blocking((
        Dataset::TaiwanStockPrice,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )) {
        Ok(v) => println!("{:?}", v),
        Err(e) => panic!("{}", e),
    }
}
