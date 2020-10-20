use chrono::{TimeZone, Utc};
use finmind::crawler;

#[cfg(test)]
use tokio_test::block_on;

#[test]
fn test_taiwan_stock_price_blocking_pass() {
    let res = crawler::taiwan_stock_price_blocking(());
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res =
        crawler::taiwan_stock_price_blocking(("0050", Utc.ymd(2020, 10, 8), Utc.ymd(2020, 10, 13)));
    match res {
        Ok(v) => {
            println!("{:?}", v.data);
            assert_eq!(v.data.len(), 3);
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_taiwan_stock_price_async_pass() {
    let res = block_on(crawler::taiwan_stock_price_async(()));
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = block_on(crawler::taiwan_stock_price_async((
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            println!("{:?}", v.data);
            assert_eq!(v.data.len(), 3);
        }
        Err(e) => assert!(false, e.to_string()),
    }
}
