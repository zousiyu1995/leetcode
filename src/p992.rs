// 维护第一个窗口，要求是存在k个不同整数的**最大区间**
// 维护第二个窗口，要求是存在k个不同整数的**最小区间**
// [1, 2, 1, 2, 3]
//     ^          满足要求的最小区间右边界
//           ^    满足要求的最大区间右边界
// 满足要求的子区间个数为r_max - r_min + 1

// 进一步简化，实际上只需要找如下两个窗口
// 存在 k 个不同整数的**最大区间**
// 存在k-1个不同整数的**最大区间**
// 满足要求的子区间个数为 r - r
pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    find_window(&nums, k) - find_window(&nums, k - 1)
}

pub fn find_window(nums: &Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans: usize = 0;
    let mut l: usize = 0;

    for r in 0..nums.len() {
        map.entry(nums[r])
            .and_modify(|v: &mut i32| *v += 1)
            .or_insert(1);
        // 窗口不满足要求
        while map.len() > k as usize {
            *map.get_mut(&nums[l]).unwrap() -= 1;
            if map[&nums[l]] == 0 {
                map.remove(&nums[l]);
            }
            l += 1;
        }
        ans += r - l + 1;
    }

    ans as i32
}

#[test]
fn test() {
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
}
