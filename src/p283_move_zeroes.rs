#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums1: Vec<i32> = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums1);
        assert_eq!(nums1, vec![1, 3, 12, 0, 0]);

        let mut nums2: Vec<i32> = vec![0];
        move_zeroes(&mut nums2);
        assert_eq!(nums2, vec![0]);
    }
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 双指针
    let mut slow: usize = 0;
    for fast in 0..nums.len() {
        // 如果fast指向的元素不是0，交换
        if nums[fast] != 0 {
            nums.swap(slow, fast); // 交换元素好像很慢
            slow += 1;
        }
        // 如果fast指向的元素是0，fast+1
    }
}
