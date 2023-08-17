#[test]
fn test() {
    assert_eq!(max_sum(vec![51, 71, 17, 24, 42]), 88);
    assert_eq!(max_sum(vec![1, 2, 3, 4]), -1);
}

// hashmap: {'max_num', [num1, num2, ...]}
// 先找数字的数位上最大的那个数字，加入哈希表
// 排序，取最后两位
pub fn max_sum(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<char, Vec<i32>> = HashMap::new();

    // 条件1：这两个数组的数位上的最大数字相等
    for num in nums {
        // 获取各数位上的最大数字，加入哈希表
        if let Some(max_num) = num.to_string().chars().max() {
            map.entry(max_num)
                .and_modify(|v| v.push(num))
                .or_insert(vec![num]);
        }
    }

    // 条件2：这两个数字的和最大
    let mut ans: i32 = -1;
    for v in map.values_mut() {
        // 数组包含2个及以上的元素
        if v.len() >= 2 {
            v.sort();
            let last2_sum: i32 = *&v[v.len() - 2..].iter().sum();
            ans = ans.max(last2_sum);
        }
        // 数组元素不够
        else {
            continue;
        }
    }

    ans
}
