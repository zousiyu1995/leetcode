// https://leetcode.cn/problems/next-greater-element-i/description/

#[test]
fn test() {
    assert_eq!(
        next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}

// 本质上是求nums2中，当前元素右侧第一个比当前元素大的元素是什么
// 单调栈+哈希表
// 和p739类似，但是本题非常绕
// 要求：数组中没有相同的元素
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut stack: Vec<i32> = vec![];
    // key：nums2的元素，value：下一个比该元素大的元素
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans: Vec<i32> = vec![];

    // 遍历nums2，生成map
    for &cur in nums2.iter() {
        // 只要栈不为空，且当前元素大于栈顶元素（即找到了下一个更大元素），更新map，出栈
        while !stack.is_empty() && cur > *stack.last().unwrap() {
            let top = *stack.last().unwrap();
            map.insert(top, cur);
            stack.pop();
        }
        // 否则，当前元素小于栈顶元素，更新map，入栈
        map.insert(cur, -1);
        stack.push(cur);
    }

    // 遍历nums1，生成ans
    for i in nums1.iter() {
        ans.push(*map.get(i).unwrap());
    }

    ans
}
