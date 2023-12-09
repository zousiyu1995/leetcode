#[test]
fn test() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sub_array(vec![1]), 1);
    assert_eq!(max_sub_array(vec![-2, 11, -4, 13, -5, -2]), 20);
}

// 动态规划
// 最大连续子序列一定以nums[i]结尾，这个序列存在两种可能
// 1. 这个序列只有一个元素，最大和是nums[i]
// 2. 这个序列有多个元素，最大和是dp[i-1] + nums[i]
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; nums.len()];
    dp[0] = nums[0];
    let mut ans: i32 = nums[0];

    // 状态转移方程
    // dp数组的最大值既是答案，生成dp的同时更新最大值
    for i in 1..nums.len() {
        dp[i] = nums[i].max(dp[i - 1] + nums[i]);
        ans = ans.max(dp[i]);
    }

    ans
}
