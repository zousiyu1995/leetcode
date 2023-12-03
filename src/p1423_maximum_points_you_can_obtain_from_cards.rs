#[test]
fn test() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    assert_eq!(max_score(vec![1, 1000, 1], 1), 1);
    assert_eq!(max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3), 202);
    assert_eq!(max_score(vec![96, 90, 41, 82, 39, 74, 64, 50, 30], 8), 536);
}

// 由于只能从头尾拿牌，所以剩下来的牌肯定是连续的
// 维护一个大小是n-k的窗口，找到这个窗口的最小值，其他被拿走的牌就能得到最大值
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n: usize = card_points.len();

    // 前缀和
    let mut presum: Vec<i32> = vec![0; n + 1];
    for i in 0..n {
        presum[i + 1] = presum[i] + card_points[i];
    }

    // 维护大小是n-k的窗口（意味着i最多可以等于k），找到窗口最小的和
    let mut min_sum: i32 = presum[n];
    for i in 0..=k as usize {
        // 窗口的和，[i, i + n - k)
        let sum: i32 = presum[i + n - k as usize] - presum[i];
        min_sum = min_sum.min(sum);
    }

    presum[n] - min_sum
}
