#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums1), 3);

        let nums2: Vec<i32> = vec![1, 2, 3];
        assert_eq!(pivot_index(nums2), -1);

        let nums3: Vec<i32> = vec![2, 1, -1];
        assert_eq!(pivot_index(nums3), 0);
    }
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut left: i32 = 0;
    for i in 0..nums.len() {
        // 如果 left == right
        if left == sum - nums[i] - left {
            return i as i32;
        }
        left += nums[i];
    }

    -1
}
