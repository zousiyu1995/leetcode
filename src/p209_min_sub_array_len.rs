#[test]
fn test() {
    let v1: Vec<i32> = vec![2, 3, 1, 2, 4, 3];
    assert_eq!(min_sub_array_len(7, v1), 2);

    let v2 = vec![1, 4, 4];
    assert_eq!(min_sub_array_len(4, v2), 1);

    let v3 = vec![1, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(min_sub_array_len(11, v3), 0);
}

// 双指针
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // 时间复杂度 O(n)
    // 空间复杂度 O(1)
    let mut sum: i32 = 0;
    let mut left: usize = 0; // 滑移窗口左指针
    let mut ans: usize = usize::MAX; // 结果窗口大小

    // 移动右指针扩大窗口
    for right in 0..nums.len() {
        sum += nums[right];
        while sum >= target {
            // 比较结果窗口和临时窗口 (right - left + 1)
            ans = ans.min(right - left + 1);
            // 左指针移动一位
            sum -= nums[left];
            left += 1;
        }
    }

    // 如果遍历完了，结果窗口的大小都没变，没找到，返回0
    match ans {
        usize::MAX => 0,
        _ => ans as i32,
    }
}
