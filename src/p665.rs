#[test]
fn test() {
    assert_eq!(check_possibility(vec![4, 2, 3]), true);
    assert_eq!(check_possibility(vec![4, 2, 1]), false);
    assert_eq!(check_possibility(vec![3, 4, 2, 3]), false);
}

// 贪心
// 不好想到
pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut nums: Vec<i32> = nums;
    let mut cnt: i32 = 0;

    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            cnt += 1;
            if i == 1 || nums[i] >= nums[i - 2] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }

    cnt <= 1
}
