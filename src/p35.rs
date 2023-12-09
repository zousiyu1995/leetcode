#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v1 = vec![1, 3, 5, 6];
        assert_eq!(search_insert(&v1, 5), 2);
        assert_eq!(search_insert(&v1, 2), 1);
        assert_eq!(search_insert(&v1, 7), 4);
    }
}

pub fn search_insert(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = nums.len();
    let mut m: usize;

    // [l, r)
    while l < r {
        m = (l + r) / 2;
        // 时刻记住，前两个if中，m不是要找的目标
        if nums[m] < target {
            l = m + 1;
        } else if nums[m] > target {
            r = m;
        } else {
            return m as i32;
        }
    }

    // 此时l=r
    l as i32
}
