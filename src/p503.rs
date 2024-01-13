// https://leetcode.cn/problems/next-greater-element-ii/description/

#[test]
fn test() {
    assert_eq!(
        next_greater_elements(vec![1, 2, 1]), //
        vec![2, -1, 2]
    );
    assert_eq!(
        next_greater_elements(vec![1, 2, 3, 4, 3]), //
        vec![2, 3, 4, -1, 4]
    );
}

// 数组是环状的，可以扩容数组到两倍大小来模拟环
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let n: usize = nums.len();
    let mut stack: Vec<usize> = vec![];
    let mut ans: Vec<i32> = vec![-1; n];

    for i in 0..(n * 2) {
        // 只要栈不为空，且当前元素大于栈顶元素
        while !stack.is_empty() && nums[i % n] > nums[*stack.last().unwrap()] {
            ans[*stack.last().unwrap()] = nums[i % n];
            stack.pop();
        }
        stack.push(i % n);
    }

    ans
}
