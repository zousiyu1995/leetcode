#[test]
fn test() {
    assert_eq!(tuple_same_product(vec![2, 3, 4, 6]), 8);
    assert_eq!(tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
}

// 用哈希表储存所有可能的积
// 假设一共有n组数的积相等，从其中挑2组，一共有C_n^2种组合
// 对于每一个组合，可以构成8个满足题意的元组
pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let n: usize = nums.len();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans: i32 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            map.entry(nums[i] * nums[j])
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    for (_k, v) in map {
        ans += (v * (v - 1) / 2) * 8;
    }

    ans
}
