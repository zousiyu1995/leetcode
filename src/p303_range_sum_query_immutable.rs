#[test]
fn test() {
    let nums: NumArray = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(nums.sum_range(0, 2), 1);
    assert_eq!(nums.sum_range(2, 5), -1);
    assert_eq!(nums.sum_range(0, 5), -3);
}

#[derive(Debug)]
#[allow(unused)]
struct NumArray {
    nums: Vec<i32>,   // 数组
    presum: Vec<i32>, // 前缀和
}

#[allow(unused)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut presum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            presum[i + 1] = presum[i] + nums[i];
        }

        NumArray {
            nums: nums,
            presum: presum,
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.presum[(right + 1) as usize] - self.presum[left as usize]
    }
}
