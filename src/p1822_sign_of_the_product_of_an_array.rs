#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![-1, -2, -3, -4, 3, 2, 1];
        assert_eq!(array_sign(nums1), 1);

        let nums2: Vec<i32> = vec![1, 5, 0, 2, -3];
        assert_eq!(array_sign(nums2), 0);

        let nums3: Vec<i32> = vec![-1, 1, -1, 1, -1];
        assert_eq!(array_sign(nums3), -1);

        let nums4: Vec<i32> = vec![
            41, 65, 14, 80, 20, 10, 55, 58, 24, 56, 28, 86, 96, 10, 3, 84, 4, 41, 13, 32, 42, 43,
            83, 78, 82, 70, 15, -41,
        ];
        assert_eq!(array_sign(nums4), -1);
    }
}

pub fn array_sign(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(1, |acc, x| {
        if x > 0 {
            acc * 1
        } else if x < 0 {
            acc * -1
        } else {
            acc * 0
        }
    })
}
