#[test]
fn test() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut presum: Vec<i32> = vec![0; nums.len()];

    presum[0] = nums[0];
    for i in 1..nums.len() {
        presum[i] = presum[i - 1] + nums[i];
    }

    presum
}
