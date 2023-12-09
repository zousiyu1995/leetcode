#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v1: Vec<i32> = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(binary_search(v1, 9), 4);

        let v2: Vec<i32> = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(binary_search(v2, 2), -1);
    }
}

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    // 左闭右开区间，l等于数组的0号索引，r比数组的最后一位索引大1
    let mut l: usize = 0;
    let mut r: usize = nums.len();

    // 检查进入二分查找的条件，这里用左闭右开区间
    while l < r {
        let m: usize = (l + r) / 2;
        if target < nums[m] {
            r = m;
        } else if target > nums[m] {
            l = m + 1;
        } else {
            return m as i32;
        }
    }

    -1
}
