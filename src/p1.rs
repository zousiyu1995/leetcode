#[test]
fn test() {
    let v1: Vec<i32> = vec![2, 7, 11, 15];
    assert_eq!(two_sum(v1, 9), vec![0, 1]);

    let v2: Vec<i32> = vec![3, 2, 4];
    assert_eq!(two_sum(v2, 6), vec![1, 2]);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.into_iter().enumerate() {
        match map.get(&(target - num)) {
            Some(&x) => return vec![x, i as i32],
            None => map.insert(num, i as i32),
        };
    }

    vec![]
}
