#[test]
fn test() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}

// 到达第n级台阶有两种方式，从第n-1级上1步，或者从第n-2级上2步
// 选哪一种取决于哪一种花费最小
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n: usize = cost.len(); // 总台阶数
    let mut dp: Vec<i32> = vec![0; n + 1]; // 记录总花费
    dp[0] = 0;
    dp[1] = 0;

    for i in 2..=n {
        // 从第i-1级台阶上来，支付i-1级台阶的费用。从第i-2级台阶上来同理。
        // 哪种方式费用少选哪种
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    }

    dp[n]
}
