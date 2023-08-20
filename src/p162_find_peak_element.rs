#[test]
fn test() {
    use method2::find_peak_element;

    assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);

    assert_eq!(find_peak_element(vec![1]), 0);
    assert_eq!(find_peak_element(vec![1, 2]), 1);
}

// 题目保证了nums[i] != nums[i+1]，最大值一定是峰值
mod method1 {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut max_idx: usize = 0;
        for (idx, &num) in nums.iter().enumerate() {
            if num > nums[max_idx] {
                max_idx = idx;
            }
        }

        max_idx as i32
    }
}

// 二分查找
// 比较m和m+1这两个位置
// 如果m<m+1，峰值在右边，缩左边界
// 如果m>m+1，峰值在左边，缩右边界
mod method2 {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // 处理特殊情况，数组长度不足2
        if nums.len() <= 1 {
            return 0;
        }

        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        // [left, right)
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] < nums[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}
