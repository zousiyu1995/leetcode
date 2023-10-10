#[test]
fn test() {
    use method2::max_profit;

    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}

// 暴力法，会超时。扫描第i天之后的股价，更新最大值
mod method1 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        for i in 0..(prices.len() - 1) {
            for j in (i + 1)..prices.len() {
                ans = ans.max(prices[j] - prices[i]);
            }
        }

        ans
    }
}

// 一次遍历，记录最低价格，同时更新最大利润
mod method2 {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_prince: i32 = i32::MAX;
        let mut max_profit: i32 = 0;
        for price in prices {
            min_prince = min_prince.min(price);
            max_profit = max_profit.max(price - min_prince);
        }

        max_profit
    }
}
