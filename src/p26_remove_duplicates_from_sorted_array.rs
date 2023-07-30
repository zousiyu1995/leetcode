#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums1: Vec<i32> = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums1), 2);

        let mut nums2: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums2), 5);
    }
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // two pointers
    let mut slow: usize = 0;
    for fast in 1..nums.len() {
        // slow == fast, fast + 1
        // slow != fast, assignment
        if nums[slow] != nums[fast] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }
    // `slow` is index, `slow + 1` is length
    (slow + 1) as i32
}
