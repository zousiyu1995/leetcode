#[test]
fn test() {
    use method2::search;

    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(search(vec![1], 0), -1);
    assert_eq!(search(vec![1], 2), -1);
    assert_eq!(search(vec![1, 3], 2), -1);
}

// 两次二分
// 先用二分找最小值，然后用二分找target
// [5, 4, 3, 2, 1]
#[allow(unused)]
mod method1 {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let last: usize = nums.len() - 1;
        // 第一次二分，数组可能是由两个有序部分合起来的，先找最小值
        let mut left: usize = 0;
        let mut right: usize = last + 1;
        // [left, right)
        while left < right {
            let mid: usize = left + (right - left) / 2;
            // 如果mid大于最后一个数，mid在最小值左侧
            if nums[mid] > nums[last] {
                left = mid + 1;
            }
            // 如果mid小于等于最后一个数，mid在最小值右侧
            else {
                right = mid;
            }
        }
        let min: usize = left;

        // 第二次二分，把数组分成两半了，每一半都是有序的，问题变成在有序数组中找target
        // 如果target>last，在[0, min)中找，[4, *5*, 1, 2, 3]
        if target > nums[last] {
            left = 0;
            right = min;
        }
        // 如果target<=last，在[min, last+1)中找，[4, 5, 1, *2*, 3]
        else {
            left = min;
            right = nums.len();
        }
        // [left, right)
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] > target {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        // 没找到
        -1
    }
}

// 一次二分
// 有点抽象
#[allow(unused)]
mod method2 {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // 判断[0, mid)是否包含target
        fn is_contain_target(mid: usize, nums: &Vec<i32>, target: i32) -> bool {
            let last: usize = nums.len() - 1;
            if nums[mid] > nums[last] {
                return target > nums[last] && nums[mid] > target;
            } else {
                return target > nums[last] || nums[mid] > target;
            }
        }

        // 二分的关键是每次找出包含target的那一段
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        while left < right {
            let mid: usize = left + (right - left) / 2;
            // 等于target
            if nums[mid] == target {
                return mid as i32;
            }
            // [0, mid)包含target
            else if is_contain_target(mid, &nums, target) {
                right = mid;
            }
            // [0, mid)不包含target
            else {
                left = mid + 1;
            }
        }

        -1
    }
}
