use chrono::{TimeZone, Utc};
use finmind::crawler;
use finmind::schema::{Data, Dataset};

#[cfg(test)]
use tokio_test::block_on;

#[test]
fn test_taiwan_stock_price_blocking_pass() {
    let res = crawler::request_blocking(Dataset::TaiwanStockPrice);
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = crawler::request_blocking((
        Dataset::TaiwanStockPrice,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            assert_eq!(v.data.len(), 3);
            for d in v.data {
                if let Data::TaiwanStockPrice(data) = d {
                    assert_eq!(data.stock_id, "0050");
                } else {
                    assert!(false, "casting TaiwanStockPrice failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_taiwan_stock_price_async_pass() {
    let res = block_on(crawler::request_async(Dataset::TaiwanStockPrice));
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockPrice,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            assert_eq!(v.data.len(), 3);
            for d in v.data {
                if let Data::TaiwanStockPrice(data) = d {
                    assert_eq!(data.stock_id, "0050");
                } else {
                    assert!(false, "casting TaiwanStockPrice failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_institutional_investors_buy_sell_blocking_pass() {
    let res = crawler::request_blocking(Dataset::InstitutionalInvestorsBuySell);
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = crawler::request_blocking((
        Dataset::InstitutionalInvestorsBuySell,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::InstitutionalInvestorsBuySell(data) = d {
                    assert_eq!(data.stock_id, "0050");
                } else {
                    assert!(false, "casting InstitutionalInvestorsBuySell failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_institutional_investors_buy_sell_async_pass() {
    let res = block_on(crawler::request_async(
        Dataset::InstitutionalInvestorsBuySell,
    ));
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = block_on(crawler::request_async((
        Dataset::InstitutionalInvestorsBuySell,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::InstitutionalInvestorsBuySell(data) = d {
                    assert_eq!(data.stock_id, "0050");
                } else {
                    assert!(false, "casting InstitutionalInvestorsBuySell failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_shareholding_blocking_pass() {
    let res = crawler::request_blocking(Dataset::Shareholding);
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = crawler::request_blocking((
        Dataset::Shareholding,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::Shareholding(data) = d {
                    assert_eq!(data.stock_id, "0050");
                } else {
                    assert!(false, "casting Shareholding failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_shareholding_async_pass() {
    let res = block_on(crawler::request_async(Dataset::Shareholding));
    match res {
        Ok(v) => assert_eq!(v.data.len(), 0),
        Err(e) => assert!(false, e.to_string()),
    }

    let res = block_on(crawler::request_async((
        Dataset::Shareholding,
        "0050",
        Utc.ymd(2020, 10, 8),
        Utc.ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::Shareholding(data) = d {
                    assert_eq!(data.stock_id, "0050");
                } else {
                    assert!(false, "casting Shareholding failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}
