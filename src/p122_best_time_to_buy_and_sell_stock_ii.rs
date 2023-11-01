#[test]
fn test() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}

// 贪心
// 在所有价格上涨的交易日都交易
// 在所有价格下降的交易日都不交易
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;

    for i in 1..prices.len() {
        // 价格上涨，能获利，就交易
        let tmp: i32 = prices[i] - prices[i - 1];
        if tmp > 0 {
            profit += tmp;
        }
    }

    profit
}
