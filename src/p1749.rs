#[test]
fn test() {
    assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
}

// 前缀和
// 子数组的和 = 两个前缀和的差
pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    // 前缀和
    let mut prefix_sum = vec![0];
    for (idx, &num) in nums.iter().enumerate() {
        prefix_sum.push(prefix_sum[idx] + num);
    }

    prefix_sum.iter().max().unwrap() - prefix_sum.iter().min().unwrap()
}
