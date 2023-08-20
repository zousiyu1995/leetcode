#[cfg(test)]
mod tests {
    use super::method1::search_range;

    #[test]
    fn test() {
        let v1: Vec<i32> = vec![5, 7, 7, 8, 8, 8, 10, 12, 15];
        search_range(v1, 8);

        // let v1: Vec<i32> = vec![5, 7, 7, 8, 8, 10];
        // assert_eq!(search_range(v1, 8), vec![3, 4]);

        // let v2: Vec<i32> = vec![5, 7, 7, 8, 8, 10];
        // assert_eq!(search_range(v2, 6), vec![-1, -1]);

        // let v3: Vec<i32> = vec![];
        // assert_eq!(search_range(v3, 0), vec![-1, -1])
    }
}
#[allow(unused)]
mod method1 {
    // 尽可能往左找的二分查找，也叫lower_bound
    pub fn binary_search(nums: &Vec<i32>, target: i32) -> usize {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        // [left, right)
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        // left = right，return任意一个
        left
    }

    // 找下界（>=target）等同于找第一个>=target的位置
    // 找上界（<=target）等同于找第一个>=(target+1)的位置，然后-1的位置
    // 因此，无论是找下界还是上界，都可以用lower_bound的二分查找解决
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start: usize = binary_search(&nums, target);
        if start == nums.len() || nums[start as usize] != target {
            return vec![-1, -1];
        }
        let end: usize = binary_search(&nums, target + 1) - 1;

        vec![start as i32, end as i32]
    }
}

#[allow(unused)]
mod method2 {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l: usize = find_left(&nums, target);
        let r: usize = find_right(&nums, target);

        if l <= r && r < nums.len() && nums[l] == target && nums[r] == target {
            return vec![l as i32, r as i32];
        } else {
            return vec![-1, -1];
        }
    }

    // 由于数组存在重复元素，因此关键在于，找到target后不能返回，而是要继续往左或右找另一个target
    pub fn find_right(nums: &Vec<i32>, target: i32) -> usize {
        let (mut l, mut r, mut ans) = (0_usize, nums.len(), nums.len());
        // [l, r)
        while l < r {
            let m: usize = l + (r - l) / 2;
            // <= 缩左边界，尽可能往右找
            if nums[m] <= target {
                l = m + 1;
                ans = m; // 标记尽可能右的答案
            } else {
                r = m;
            }
        }
        ans
    }

    pub fn find_left(nums: &Vec<i32>, target: i32) -> usize {
        let (mut l, mut r, mut ans) = (0_usize, nums.len(), nums.len());
        // [l, r)
        while l < r {
            let m: usize = l + (r - l) / 2;
            // >= 缩右边界，尽可能往左找
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
                ans = m; // 标记尽可能左的答案
            }
        }
        ans
    }
}
