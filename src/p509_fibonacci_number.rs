#[test]
fn test() {
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
}

// 动态规划
pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
    dp[0] = 0;
    dp[1] = 1;

    // 递推
    for i in 2..=(n as usize) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n as usize]
}
