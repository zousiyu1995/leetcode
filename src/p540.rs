#[test]
fn test() {
    use method2::single_non_duplicate;

    assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    // assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
    // assert_eq!(single_non_duplicate(vec![1]), 1);
}

// 暴力
// 时间复杂度O(N)
mod method1 {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut i: usize = 0;

        while i < n - 1 {
            if nums[i] != nums[i + 1] {
                return nums[i];
            }
            i += 2;
        }

        nums[n - 1]
    }
}

// 二分查找，关键是二段性
// 时间复杂度O(logN)
// 这题细节没搞懂，二分查找的边界问题很烦人
mod method2 {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut l: usize = 0;
        let mut r: usize = n;

        // [l, r)
        while l < r {
            let mid: usize = l + (r - l) / 2;
            // 如果mid下标是偶数
            if mid % 2 == 0 {
                // 先判断mid + 1是否合法。如果mid == mid + 1，那么mid之前肯定没有单一元素
                if mid + 1 < n && nums[mid] == nums[mid + 1] {
                    l = mid + 1;
                }
                // 如果mid != mid + 1，mid可能是单一元素，为了不错过mid，要注意r的更新策略
                else {
                    r = mid;
                }
            }
            // 如果mid下标是奇数
            else {
                // 先判断mid - 1是否合法。如果mid = mid - 1，那么mid之前肯定没有单一元素
                if mid >= 1 && nums[mid] == nums[mid - 1] {
                    l = mid + 1;
                }
                // 如果mid !+ mid - 1，mid可能是单一元素，为了不错过mid，要注意r的更新策略
                else {
                    r = mid;
                }
            }
        }

        nums[l]
    }
}
