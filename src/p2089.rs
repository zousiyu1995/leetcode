#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![1, 2, 5, 2, 3];
        assert_eq!(target_indices(nums1, 2), vec![1, 2]);

        let nums2: Vec<i32> = vec![1, 2, 5, 2, 3];
        assert_eq!(target_indices(nums2, 3), vec![3]);

        let nums3: Vec<i32> = vec![1, 2, 5, 2, 3];
        assert_eq!(target_indices(nums3, 5), vec![4]);

        let nums4: Vec<i32> = vec![1, 2, 5, 2, 3];
        assert_eq!(target_indices(nums4, 4), vec![]);
    }
}

pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    nums.sort();
    nums.iter().enumerate().for_each(|(idx, x)| {
        if *x == target {
            ans.push(idx as i32)
        }
    });

    ans
}
