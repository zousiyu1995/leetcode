#[test]
fn test() {
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    assert_eq!(find_min(vec![0]), 0);
}

// 二分查找
// 按题意可知数组的最后一个数的性质
// 其要么是最小值：[5, 4, 3, 2, 1]
// 要么在最小值右侧：[4, 5, 1, 2, 3]或[1, 2, 3, 4, 5]
// 因此，二分的比较条件是和数组的最后一个数比
// 条件：数组长度[1, 5000]
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = nums.len();
    // [left, right)
    while left < right {
        let mid: usize = left + (right - left) / 2;
        // 如果mid大于最后一个数，mid在最小值左侧
        if nums[mid] > nums[nums.len() - 1] {
            left = mid + 1;
        }
        // 如果mid小于等于最后一个数，mid在最小值右侧
        else {
            right = mid;
        }
    }

    nums[left]
}
