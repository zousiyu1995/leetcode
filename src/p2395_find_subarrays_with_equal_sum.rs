#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![4, 2, 4];
        assert_eq!(find_subarrays(nums1), true);

        let nums2: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(find_subarrays(nums2), false);

        let nums3: Vec<i32> = vec![0, 0, 0];
        assert_eq!(find_subarrays(nums3), true);
    }
}

use std::collections::HashSet;

pub fn find_subarrays(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..nums.len() - 1 {
        let sum: i32 = nums[i] + nums[i + 1];
        // If the set already contained this value, `false` is returned.
        if !set.insert(sum) {
            return true;
        }
    }

    false
}
