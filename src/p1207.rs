#[test]
fn test() {
    let arr1: Vec<i32> = vec![1, 2, 2, 1, 1, 3];
    assert_eq!(unique_occurrences(arr1), true);

    let arr2: Vec<i32> = vec![1, 2];
    assert_eq!(unique_occurrences(arr2), false);

    let arr3: Vec<i32> = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
    assert_eq!(unique_occurrences(arr3), true);
}

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::{HashMap, HashSet};

    // 统计arr中数字出现的次数
    let mut map: HashMap<i32, i32> = HashMap::new();
    arr.iter()
        .for_each(|&num| *map.entry(num).or_insert(0) += 1);

    // 统计次数是否重复
    map.len() == map.values().collect::<HashSet<_>>().len()
}
