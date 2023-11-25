#[test]
fn test() {
    assert_eq!(remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
    assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
}

// 对当前遍历的数nums[i]来讲，将它与它前面k个位置的数nums[i-k]做比较
// 如果两者不同，保留；否则，跳过
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut w: usize = 0; // 写指针
    let k: usize = 2; // 保留重复的k个数

    // 前k个数总是保留
    for i in 0..nums.len() {
        if i < k || nums[i] != nums[w - k] {
            nums[w] = nums[i];
            w += 1;
        }
    }

    w as i32
}
