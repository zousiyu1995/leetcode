#[test]
fn test() {
    assert_eq!(min_operations(vec![3, 1, 5, 4, 2], 2), 4);
    assert_eq!(min_operations(vec![3, 1, 5, 4, 2], 5), 5);
    assert_eq!(min_operations(vec![3, 2, 5, 3, 1], 3), 4);
}

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    let mut count: i32 = 0;

    for i in (0..nums.len()).rev() {
        count += 1;
        // 只关心大小在[1, k]的数
        if nums[i] <= k {
            set.insert(nums[i]);
            if set.len() == k as usize {
                break;
            }
        }
    }

    count
}
