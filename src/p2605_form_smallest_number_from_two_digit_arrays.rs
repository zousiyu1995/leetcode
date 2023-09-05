#[test]
fn test() {
    assert_eq!(min_number(vec![4, 1, 3], vec![5, 7]), 15);
    assert_eq!(min_number(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
}

// 条件：每个数组中，元素互不相同
// 哈希集
// 如果两个数组有相同的数字，返回该数字，该数字是个位数，肯定最小
// 否则，返回各自数组最小的数字组成答案
pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let set1: HashSet<&i32> = HashSet::from_iter(nums1.iter());
    let set2: HashSet<&i32> = HashSet::from_iter(nums2.iter());

    if let Some(&&v) = set1.intersection(&set2).min() {
        return v;
    }

    let x: i32 = *nums1.iter().min().unwrap();
    let y: i32 = *nums2.iter().min().unwrap();

    (x * 10 + y).min(x + y * 10)
}
