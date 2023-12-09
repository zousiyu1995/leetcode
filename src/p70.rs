#[test]
fn test() {
    assert_eq!(climb_stairs(1), 1);
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}

// 动态规划
// 爬上第n级楼梯有两种爬法，一种是从第n-1级爬1步上来，另一种是从第n-2级爬2步上来
pub fn climb_stairs(n: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=(n as usize) {
        dp[i] = dp[i - 2] + dp[i - 1];
    }

    dp[n as usize]
}
