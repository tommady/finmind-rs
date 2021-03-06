use chrono::NaiveDate;
use finmind::crawler;
use finmind::schema::{Data, Dataset};

#[cfg(test)]
use tokio_test::block_on;

#[test]
fn test_taiwan_stock_price_blocking_pass() {
    let res = crawler::request_blocking((
        Dataset::TaiwanStockPrice,
        "0050".to_owned().to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            assert_eq!(v.data.len(), 3);
            for d in v.data {
                if let Data::TaiwanStockPrice(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
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
    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockPrice,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            assert_eq!(v.data.len(), 3);
            for d in v.data {
                if let Data::TaiwanStockPrice(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
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
    let res = crawler::request_blocking((
        Dataset::TaiwanStockInstitutionalInvestorsBuySell,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockInstitutionalInvestorsBuySell(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
                } else {
                    assert!(
                        false,
                        "casting TaiwanStockInstitutionalInvestorsBuySell failed"
                    );
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_institutional_investors_buy_sell_async_pass() {
    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockInstitutionalInvestorsBuySell,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockInstitutionalInvestorsBuySell(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
                } else {
                    assert!(
                        false,
                        "casting TaiwanStockInstitutionalInvestorsBuySell failed"
                    );
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_shareholding_blocking_pass() {
    let res = crawler::request_blocking((
        Dataset::TaiwanStockShareholding,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockShareholding(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
                } else {
                    assert!(false, "casting TaiwanStockShareholding failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_shareholding_async_pass() {
    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockShareholding,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockShareholding(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
                } else {
                    assert!(false, "casting TaiwanStockShareholding failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_taiwan_stock_margin_purchase_short_sale_blocking_pass() {
    let res = crawler::request_blocking((
        Dataset::TaiwanStockMarginPurchaseShortSale,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockMarginPurchaseShortSale(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
                } else {
                    assert!(false, "casting TaiwanStockMarginPurchaseShortSale failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_taiwan_stock_margin_purchase_short_sale_async_pass() {
    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockMarginPurchaseShortSale,
        "0050".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockMarginPurchaseShortSale(data) = d {
                    assert_eq!(data.stock_id, "0050".to_owned());
                } else {
                    assert!(false, "casting TaiwanStockMarginPurchaseShortSale failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_taiwan_stock_month_revenue_blocking_pass() {
    let res = crawler::request_blocking((
        Dataset::TaiwanStockMonthRevenue,
        "2330".to_owned().to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockMonthRevenue(data) = d {
                    assert_eq!(data.stock_id, "2330".to_owned());
                } else {
                    assert!(false, "casting TaiwanStockMonthRevenue failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_taiwan_stock_month_revenue_async_pass() {
    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockMonthRevenue,
        "2330".to_owned(),
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockMonthRevenue(data) = d {
                    assert_eq!(data.stock_id, "2330".to_owned());
                } else {
                    assert!(false, "casting TaiwanStockMonthRevenue failed");
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_institutional_investors_blocking_pass() {
    let res = crawler::request_blocking((
        Dataset::TaiwanStockTotalInstitutionalInvestors,
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    ));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockTotalInstitutionalInvestors(data) = d {
                    assert_ne!(data.name, "".to_owned());
                } else {
                    assert!(
                        false,
                        "casting TaiwanStockTotalInstitutionalInvestors failed"
                    );
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_institutional_investors_async_pass() {
    let res = block_on(crawler::request_async((
        Dataset::TaiwanStockTotalInstitutionalInvestors,
        NaiveDate::from_ymd(2020, 10, 8),
        NaiveDate::from_ymd(2020, 10, 13),
    )));
    match res {
        Ok(v) => {
            for d in v.data {
                if let Data::TaiwanStockTotalInstitutionalInvestors(data) = d {
                    assert_ne!(data.name, "".to_owned());
                } else {
                    assert!(
                        false,
                        "casting TaiwanStockTotalInstitutionalInvestors failed"
                    );
                }
            }
        }
        Err(e) => assert!(false, e.to_string()),
    }
}
