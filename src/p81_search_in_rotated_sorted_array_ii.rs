#[test]
fn test() {
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}

// 二分查找的关键是二段性
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut l: usize = 0;
    let mut r: usize = nums.len();

    // [l, r)
    while l < r {
        let mid: usize = l + (r - l) / 2;

        if nums[mid] == target {
            return true;
        }

        // 碰到重复的数字了，无法判断哪个区间有序
        if nums[l] == nums[mid] {
            l += 1;
            continue;
        }

        // 前半段是有序的
        if nums[l] < nums[mid] {
            // l <= target < mid
            if target >= nums[l] && target < nums[mid] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        // 后半段是有序的
        else {
            // mid < target <= r
            if target > nums[mid] && target <= nums[r - 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
    }

    false
}
